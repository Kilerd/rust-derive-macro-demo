use proc_macro::{TokenStream};
use syn::{parse_macro_input, parse2, DeriveInput};
use proc_macro2::Ident;
use quote::quote;
use darling::FromDeriveInput;
use darling::FromVariant;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(detail), supports(enum_any))]
struct DetailErrorEnum {
    ident: syn::Ident,
    data: darling::ast::Data<DetailErrorVariant, darling::util::Ignored>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(detail))]
struct DetailErrorVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<syn::Field>,
    #[darling(default)]
    code: Option<u16>,
    #[darling(default)]
    message: Option<String>,
}

#[proc_macro_derive(DetailError, attributes(detail))]
pub fn detail_error_fn(input: TokenStream) -> TokenStream {
    handler(input.into()).into()
}

fn handler(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let result = parse2::<DeriveInput>(input).unwrap();
    let detail_error: DetailErrorEnum = DetailErrorEnum::from_derive_input(&result).unwrap();

    dbg!(&detail_error);

    let ident = &detail_error.ident;
    let variants = detail_error.data.take_enum().unwrap();
    let http_code_fn_codegen: Vec<proc_macro2::TokenStream> = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let http_code = variant.code.unwrap_or(400);

        quote! {
            #ident::#variant_ident => #http_code
        }
    }).collect();
    let code_fn_codegen: Vec<proc_macro2::TokenStream> = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let content = inflector::cases::screamingsnakecase::to_screaming_snake_case(&variant_ident.to_string());
        quote! {
            #ident::#variant_ident => String::from(#content)
        }
    }).collect();

    let message_fn_codegen: Vec<proc_macro2::TokenStream> = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let message = variant.message.clone().unwrap_or_else(|| {
            inflector::cases::sentencecase::to_sentence_case(&variant_ident.to_string())
        });
        quote! {
            #ident::#variant_ident => String::from(#message)
        }
    }).collect();

    let output = quote! {
        impl #ident {
            pub fn get_http_code(&self) -> u16 {
                match self {
                    #(#http_code_fn_codegen,)*
                }
            }
            pub fn get_code(&self) -> String {
                match self {
                    #(#code_fn_codegen,)*
                }
            }
            pub fn get_message(&self) -> String {
                match self {
                    #(#message_fn_codegen,)*
                }
            }
        }
    };
    output
}

#[cfg(test)]
mod tests {
    use crate::handler;
    use quote::quote;

    #[test]
    fn it_works() {
        let input = quote! {
            pub enum A {
                A,

            }
        };
        let expected_output = quote! {
            impl A {
                pub fn get_http_code(&self) -> u16 {
                    match self {
                        A::A => 400u16,
                    }
                }
                pub fn get_code(&self) -> String {
                    match self {
                        A::A => String::from("A"),
                    }
                }
                pub fn get_message(&self) -> String {
                    match self {
                        A::A => String::from("A"),
                    }
                }
            }
        };
        let output = handler(input);
        assert_eq!(expected_output.to_string(), output.to_string());
    }
}
