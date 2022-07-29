use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::Item;
use syn::Type::Reference;

use crate::{METHOD_GET_PREFIX, METHOD_SET_PREFIX};

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
                        let return_type = match &field.ty {
                            Reference(ty) => ty.to_token_stream(),
                            _ => quote! {&#field_type}
                        };
                        let set_method_name = format_ident!("{}{}",METHOD_SET_PREFIX,field_name);
                        let get_method_name = format_ident!("{}{}",METHOD_GET_PREFIX,field_name);
                        quote! {
                            pub fn #get_method_name(&self) -> #return_type {
                                &self.#field_name
                            }
                            pub fn #set_method_name(&mut self, #field_name: #field_type) -> &mut Self{
                                self.#field_name = #field_name;
                                self
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
                syn::Fields::Unnamed(fields_unnamed) => Err(syn::Error::new_spanned(fields_unnamed, "#[derive(Accessors)] doesn't support tuple struct")),
                syn::Fields::Unit => Ok(quote! {}),
            }
        }
        _ => Err(syn::Error::new_spanned(item, "#[derive(Accessors)] just support struct")),
    }
}