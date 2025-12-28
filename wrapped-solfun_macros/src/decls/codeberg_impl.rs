macro_rules! codeberg_impl {
    () => {
        # [decl (fn , name = "codeberg_impl" , vis = "pub" , hash = "0d4c7787")] pub fn codeberg_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🦊 CODEBERG! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
    };
}

codeberg_impl!();