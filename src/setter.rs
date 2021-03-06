use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Item;

use crate::METHOD_SET_PREFIX;

pub(crate) fn expand(item: Item) -> syn::Result<TokenStream> {
    match item {
        syn::Item::Struct(item) => {
            let syn::ItemStruct {
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
                        let method_name = format_ident!("{}{}",METHOD_SET_PREFIX,field_name);
                        quote! {
                    pub fn #method_name(&mut self, #field_name: #field_type){
                        self.#field_name = #field_name
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
                syn::Fields::Unnamed(fields_unnamed) => Err(syn::Error::new_spanned(fields_unnamed, "#[derive(Setter)] doesn't support tuple struct")),
                syn::Fields::Unit => Ok(quote! {}),
            }
        }
        _ => Err(syn::Error::new_spanned(item, "#[derive(Setter)] just support struct")),
    }
}