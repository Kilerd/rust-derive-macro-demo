use proc_macro::{TokenStream};
use syn::parse_macro_input;
use proc_macro2::Ident;
use quote::quote;

#[proc_macro_derive(DetailError, attributes(detail))]
pub fn detail_error_fn(input: TokenStream) -> TokenStream {
    let enum_struct = parse_macro_input!(input as syn::ItemEnum);
    dbg!(&enum_struct);
    let ident = &enum_struct.ident;
    let variants_ident: Vec<&Ident> = enum_struct.variants.iter().map(|variant| &variant.ident).collect();
    let code_fn_codegen: Vec<proc_macro2::TokenStream> = enum_struct.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let content = inflector::cases::screamingsnakecase::to_screaming_snake_case(&variant_ident.to_string());
        quote! {
            #ident::#variant_ident => String::from(#content)
        }
    }).collect();
    let message_fn_codegen: Vec<proc_macro2::TokenStream> = enum_struct.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let content = inflector::cases::sentencecase::to_sentence_case(&variant_ident.to_string());
        quote! {
            #ident::#variant_ident => String::from(#content)
        }
    }).collect();

    let output = quote! {
        impl #ident {
            pub fn get_http_code(&self) -> u16 {
                match self {
                    #(#ident::#variants_ident => 400,)*
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
    output.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
