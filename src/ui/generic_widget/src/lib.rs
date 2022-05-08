extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemStruct, NestedMeta};

/// It takes a struct and a generic attribute, and returns a struct with the generic attribute applied
///
/// Arguments:
///
/// * `args`: TokenStream - The arguments passed to the macro.
/// * `input`: TokenStream - The input to the macro.
///
/// Returns:
///
/// The TokenStream is being returned.
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let ident = args
        .iter()
        .find_map(|arg| match arg {
            NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(path) => path.get_ident(),
                _ => panic!("expected path"),
            },
            _ => panic!("expected meta"),
        })
        .unwrap();

    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as ItemStruct);

    // Build the output token stream.
    let generic_widget = match ident.to_string().as_str() {
        "Button" => {
            quote! {
                #[generic_button]
            }
        }
        _ => unreachable!(),
    };
    TokenStream::from(quote! {
        #[generic_policy]
        #generic_widget
        #ast
    })
}

/// It takes a struct and generates a policy struct for it
///
/// Arguments:
///
/// * `_args`: TokenStream
/// * `input`: TokenStream - The input to the macro.
///
/// Returns:
///
/// The TokenStream is being returned.
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic_policy(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    let name = &ast.ident;
    let policy = format_ident!("{}Policy", name);
    let generic_policy = quote! {
        #[derive(Component, Debug, Default, Clone, PartialEq, Eq, Hash)]
        pub struct #policy {}

    };
    TokenStream::from(quote! { #generic_policy #ast })
}

/// It takes a struct as user-defined button widget and adds a bunch of Component required by Bevy ui to it
///
/// Arguments:
///
/// * `_args`: TokenStream
/// * `input`: TokenStream - The input to the macro.
///
/// Returns:
///
/// The TokenStream is a sequence of tokens that is returned by the macro.
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic_button(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as ItemStruct);

    // Build the output, possibly using quasi-quotation.
    let name = &ast.ident;
    let policy = format_ident!("{}Policy", name);
    let comments = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("doc"))
        .fold(quote!(), |acc, attr| {
            quote! {
                #acc
                #attr
            }
        });
    let derives = ast.attrs.iter().find(|attr| attr.path.is_ident("derive"));
    let visiblity = &ast.vis;
    let content = match ast.fields {
        syn::Fields::Named(ref fields) => {
            let fields = fields.named.iter().map(|field| {
                let ident = &field.ident;
                let ty = &field.ty;
                let visiblity = &field.vis;
                let comments = field
                    .attrs
                    .iter()
                    .filter(|attr| attr.path.is_ident("doc"))
                    .fold(quote!(), |acc, attr| {
                        quote! {
                            #acc
                            #attr
                        }
                    });
                quote! {
                    #comments
                    #visiblity #ident: #ty,
                }
            });
            quote! {
                #(#fields)*
            }
        }
        _ => panic!("expected named fields"),
    };

    let generic_button = quote! {
        /// Describes the size of the node
        pub node: Node,
        /// Marker component that signals this node is a button
        pub button: Button,
        /// Describes the style including flexbox settings
        pub style: Style,
        /// Describes whether and how the button has been interacted with by the input
        pub interaction: Interaction,
        /// The color of the node
        pub color: UiColor,
        /// The image of the node
        pub image: UiImage,
        /// The transform of the node
        pub transform: Transform,
        /// The global transform of the node
        pub global_transform: GlobalTransform,
        /// Describes the visibility properties of the node
        pub visibility: Visibility,
        /// Policy derived from `#name`
        pub policy: #policy,
    };

    let ast = quote! {
        #comments
        #derives
        #visiblity struct #name {
            #generic_button
            #content
        }
    };
    TokenStream::from(ast)
}
