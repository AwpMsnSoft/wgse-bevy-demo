extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
#[allow(unreachable_code)]
pub fn generic_button(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as ItemStruct);
    let name = &ast.ident;
    let derives = ast
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("derive"))
        .unwrap();
    let visiblity = &ast.vis;
    let content = match ast.fields {
        syn::Fields::Named(ref fields) => {
            let fields = fields.named.iter().map(|field| {
                let ident = &field.ident;
                let r#type = &field.ty;
                let visiblity = &field.vis;
                quote! {
                    #visiblity #ident: #r#type,
                }
            });
            quote! {
                #(#fields), *
            }
        }
        _ => panic!("#[generic_button] only supports named fields"),
    };
    TokenStream::from(quote! {
        #derives
        #visiblity struct #name {
            /// Describes the size of the node
            pub node: Node,
            /// Marker component that signals this node is a button
            pub button: Button,
            /// Describes the style including flexbox settings
            pub style: Style,
            /// Describes whether and how the button has been interacted with by the input
            pub interaction: Interaction,
            /// Whether this node should block interaction with lower nodes
            pub focus_policy: FocusPolicy,
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
            /// User define components
            #content
        }
    })
}
