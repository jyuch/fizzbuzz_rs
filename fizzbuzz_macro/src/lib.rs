use proc_macro::TokenStream;

#[proc_macro]
pub fn fizzbuzz(item: TokenStream) -> TokenStream {
    fizzbuzz_macro_internals::fizzbuzz(item.into()).into()
}
