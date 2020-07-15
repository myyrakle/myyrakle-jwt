#[proc_macro_derive(Payload)]
pub fn derive_payload(item: TokenStream) -> TokenStream {
    let mut code = item.to_string();
    println!("{}", code);
    item
}
