use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AsStd140)]
pub fn std140_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(data_struct) => {
            let write_calls = data_struct.fields.into_iter().map(|field| {
                let field_name = field.ident;

                quote! {
                    buf.write(&self.#field_name);
                }
            });

            quote! {
                impl gpu_bytes::AsStd140 for #name {
                    fn as_std140(&self) -> gpu_bytes::Std140Bytes {
                        let mut buf = gpu_bytes::Std140Bytes::new();

                        #(
                            #write_calls
                        )*

                        buf
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(_) => syn::Error::new_spanned(name, "AsStd140 cannot be derived for enums")
            .to_compile_error()
            .into(),
        syn::Data::Union(_) => {
            syn::Error::new_spanned(name, "AsStd140 cannot be derived for unions")
                .to_compile_error()
                .into()
        }
    }
}

#[proc_macro_derive(AsStd430)]
pub fn std430_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    match input.data {
        syn::Data::Struct(data_struct) => {
            let write_calls = data_struct.fields.into_iter().map(|field| {
                let field_name = field.ident;

                quote! {
                    buf.write(&self.#field_name);
                }
            });

            quote! {
                impl gpu_bytes::AsStd430 for #name {
                    fn as_std430(&self) -> gpu_bytes::Std430Bytes {
                        let mut buf = gpu_bytes::Std430Bytes::new();

                        #(
                            #write_calls
                        )*

                        buf
                    }
                }
            }
            .into()
        }
        syn::Data::Enum(_) => syn::Error::new_spanned(name, "AsStd430 cannot be derived for enums")
            .to_compile_error()
            .into(),
        syn::Data::Union(_) => {
            syn::Error::new_spanned(name, "AsStd430 cannot be derived for unions")
                .to_compile_error()
                .into()
        }
    }
}
