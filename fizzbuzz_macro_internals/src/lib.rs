use proc_macro2::TokenStream;
use quote::quote;

pub fn fizzbuzz(_items: TokenStream) -> TokenStream {
    quote! { println!("Hello world") }
}
