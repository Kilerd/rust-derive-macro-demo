
use proc_macro::TokenStream;

#[proc_macro_derive(DetailError, attributes(detail))]
pub fn detail_error_fn(input: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
