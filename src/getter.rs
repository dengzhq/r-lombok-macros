use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Item;
use syn::Type::Reference;

use crate::METHOD_GET_PREFIX;

pub(crate) fn expand(item: Item) -> syn::Result<TokenStream> {
    match item {
        syn::Item::Struct(item) => {
            let syn::ItemStruct {
                attrs: _attrs,
                ident,
                generics,
                fields,
                ..
            } = item;
            match fields {
                syn::Fields::Named(fields_named) => {
                    let impl_content = fields_named.named.iter().map(|field| {
                        let field_name = field.ident.clone().unwrap();
                        let field_type = &field.ty;
                        let return_type = match &field.ty {
                            Reference(ty) => ty.to_token_stream(),
                            _ => quote! {&#field_type}
                        };
                        let method_name = format_ident!("{}{}",METHOD_GET_PREFIX,field_name);
                        quote! {
                            pub fn #method_name(&self) -> #return_type {
                                &self.#field_name
                            }
                        }
                    });
                    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                    Ok(quote! {
                        impl #impl_generics #ident #ty_generics #where_clause {
                                 #(#impl_content)*
                        }
                    })
                }
                syn::Fields::Unnamed(fields_unnamed) => Err(syn::Error::new_spanned(fields_unnamed, "#[derive(Getter)] doesn't support tuple struct")),
                syn::Fields::Unit => Ok(quote! {}),
            }
        }
        _ => Err(syn::Error::new_spanned(item, "#[derive(Getter)] just support struct")),
    }
}