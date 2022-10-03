use proc_macro2::TokenStream;
use quote::quote;

pub fn fizzbuzz(items: TokenStream) -> TokenStream {
    let n = (format!("{}", items)).parse::<u32>().unwrap();

    let mut prints = Vec::new();

    for i in 1..=n {
        let out: TokenStream = if i % 15 == 0 {
            "\"fizzbuzz\"".parse().unwrap()
        } else if i % 5 == 0 {
            "\"buzz\"".parse().unwrap()
        } else if i % 3 == 0 {
            "\"fizz\"".parse().unwrap()
        } else {
            format!("{}", i).parse().unwrap()
        };
        prints.push(Box::new(quote!(println!("{}", #out);)));
    }

    quote! { #(#prints)* }
}
