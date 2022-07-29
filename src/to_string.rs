use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

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
                    let format_content = fields_named.named.iter().map(|field| {
                        let field_name = field.ident.clone().unwrap();
                        let field_name_string = field.ident.clone().unwrap().to_string();
                        quote! {
                            builder.field(#field_name_string, &&self.#field_name);
                        }
                    });
                    let struct_name_string = ident.to_string();
                    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
                    Ok(quote! {
                                impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
                                    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        let mut builder = fmt.debug_struct(#struct_name_string);
                                        #(
                                            #format_content
                                        )*
                                        builder.finish()
                                    }
                                }

                                impl #impl_generics ::core::fmt::Display for #ident #ty_generics #where_clause {
                                    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        self.fmt(fmt)
                                    }
                                }

                                impl #impl_generics #ident #ty_generics #where_clause {
                                    fn to_string(&self) -> ::std::string::String {
                                        ::std::format!("{:?}", self)
                                    }
                                }
                    })
                }
                syn::Fields::Unnamed(fields_unnamed) => Err(syn::Error::new_spanned(fields_unnamed, "#[derive(ToString)] doesn't support tuple struct")),
                syn::Fields::Unit => Ok(quote! {}),
            }
        }
        _ => Err(syn::Error::new_spanned(item, "#[derive(ToString)] just support struct")),
    }
}