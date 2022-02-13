use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn runtime_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_fn = parse_macro_input!(item as syn::ItemFn);

    let result = quote! {
        #parsed_fn
    };
    result.into()
}
