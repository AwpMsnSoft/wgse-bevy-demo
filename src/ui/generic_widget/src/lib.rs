extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, Item, ItemStruct};

/// Parse comments in AST and return a TokenStream
macro_rules! get_comments {
    ($attrs: expr) => {{
        $attrs
            .iter()
            .filter(|attr| attr.path.is_ident("doc"))
            .fold(quote!(), |acc, attr| {
                quote! {
                    #acc
                    #attr
                }
            })
    }};
}

/// Parse derives in AST and return a TokenStream
macro_rules! get_derive {
    ($attrs: expr) => {
        $attrs.iter().find(|attr| attr.path.is_ident("derive"))
    };
}

/// Parse AST of a struct like
/// ```
/// #comments
/// #derives
/// #visibility struct #name
/// {
///     #contents
/// }
/// ```
/// 
/// Returns: 
///     `(comments, derives, visibility, name, contents)`
macro_rules! parse_struct {
    ($ast: expr) => {{
        let name = &$ast.ident;
        let comments = get_comments!(&$ast.attrs);
        let derives = get_derive!(&$ast.attrs);
        let visiblity = &$ast.vis;
        let content = match $ast.fields {
            syn::Fields::Named(ref fields) => {
                let fields = fields.named.iter().map(|field| {
                    let ident = &field.ident;
                    let ty = &field.ty;
                    let visiblity = &field.vis;
                    let comments = get_comments!(&field.attrs);
                    quote! {
                        #comments
                        #visiblity #ident: #ty,
                    }
                });
                quote! {
                    #(#fields)*
                }
            }
            syn::Fields::Unnamed(ref fields) => {
                let fields = fields.unnamed.iter().map(|field| {
                    let ty = &field.ty;
                    let visiblity = &field.vis;
                    let comments = get_comments!(&field.attrs);
                    quote! {
                        #comments
                        #visiblity #ty,
                    }
                });
                quote! {
                    #(#fields)*
                }
            }
            syn::Fields::Unit => quote! {},
        };
        (comments, derives, visiblity, name, content)
    }};
}

/// This macro is a simplify of the `#[derive(Debug, Clone, Component, ...)]` macro.
///
/// # Examples
/// ```
/// #[component(Default, Deref)]
/// pub struct UiWidgetId1(pub i32);
///
/// // The struct `UiWidgetId2` is equivalent to `UiWidgetId1`:
/// #[derive(Debug, Clone, Component, Default, Deref)]
/// pub struct UiWidgetId2(pub i32);
/// ```
///
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn component(args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as Item);
    let args = parse_macro_input!(args as AttributeArgs);
    let ident = {
        let ident = args.iter();
        quote! { #(#ident),* }
    };
    TokenStream::from(quote! {
        #[derive(Debug, Clone, Component, #ident)]
        #ast
    })
}

/// This macro adds default components defined by `bevy_ui` and a `UiWidgetId` member into user-defined bundle.
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    assert_eq!(args.len(), 1);
    let ident = {
        let ident = &args[0];
        quote! { #ident }
    };

    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as ItemStruct);
    let (comments, derives, visiblity, name, content) = parse_struct!(ast);

    // Build the output token stream.
    let generic_widget = match ident.to_string().as_str() {
        "Button" => {
            quote! {
                #[generic_button]
            }
        }
        "Text" => {
            quote! {
                #[generic_text]
            }
        }
        "Image" => {
            quote! {
                #[generic_image]
            }
        }
        _ => unreachable!(),
    };
    TokenStream::from(quote! {
        #generic_widget
        #comments
        #derives
        #visiblity struct #name {
            /// The unique identifier of the widget.
            pub(crate) widget_id: UiWidgetId,
            #content
        }
    })
}

/// It takes a struct as user-defined button widget and adds a bunch of Component required by Bevy ui to it
///
/// According to the `bevy_ui` source code, a `Button` widget should have the following components:
/// ```
/// /// A UI node that is a button
/// #[derive(Bundle, Clone, Debug)]
/// pub struct ButtonBundle {
///     /// Describes the size of the node
///     pub node: Node,
///     /// Marker component that signals this node is a button
///     pub button: Button,
///     /// Describes the style including flexbox settings
///     pub style: Style,
///     /// Describes whether and how the button has been interacted with by the input
///     pub interaction: Interaction,
///     /// Whether this node should block interaction with lower nodes
///     pub focus_policy: FocusPolicy,
///     /// The color of the node
///     pub color: UiColor,
///     /// The image of the node
///     pub image: UiImage,
///     /// The transform of the node
///     pub transform: Transform,
///     /// The global transform of the node
///     pub global_transform: GlobalTransform,
///     /// Describes the visibility properties of the node
///     pub visibility: Visibility,
/// }
/// ```
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic_button(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as ItemStruct);

    // Build the output, possibly using quasi-quotation.
    let (comments, derives, visiblity, name, content) = parse_struct!(ast);

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

/// It takes a struct as user-defined text widget and adds a bunch of Component required by Bevy ui to it
///
/// According to the `bevy_ui` source code, a `Text` widget should have the following components:
/// ```
/// /// A UI node that is text
/// #[derive(Bundle, Clone, Debug)]
/// pub struct TextBundle {
///     /// Describes the size of the node
///     pub node: Node,
///     /// Describes the style including flexbox settings
///     pub style: Style,
///     /// Contains the text of the node
///     pub text: Text,
///     /// The calculated size based on the given image
///     pub calculated_size: CalculatedSize,
///     /// Whether this node should block interaction with lower nodes
///     pub focus_policy: FocusPolicy,
///     /// The transform of the node
///     pub transform: Transform,
///     /// The global transform of the node
///     pub global_transform: GlobalTransform,
///     /// Describes the visibility properties of the node
///     pub visibility: Visibility,
/// }
/// ```
#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic_text(_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree.
    let ast = parse_macro_input!(input as ItemStruct);

    // Build the output, possibly using quasi-quotation.
    let (comments, derives, visiblity, name, content) = parse_struct!(ast);

    let generic_text = quote! {
        /// Describes the size of the node
        pub node: Node,
        /// Describes the style including flexbox settings
        pub style: Style,
        /// Contains the text of the node
        pub text: Text,
        /// The calculated size based on the Fiven image
        pub calculated_size: CalculatedSize,
        /// Whether this node should block interaction with lower nodes
        pub focus_policy: FocusPolicy,
        /// The transform of the node
        pub transform: Transform,
        /// The global transform of the node
        pub global_transform: GlobalTransform,
        /// Describes the visibility properties of the node
        pub visibility: Visibility,
    };

    let ast = quote! {
        #comments
        #derives
        #visiblity struct #name {
            #generic_text
            #content
        }
    };
    TokenStream::from(ast)
}
