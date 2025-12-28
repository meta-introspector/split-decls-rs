macro_rules! telegram_impl {
    () => {
        # [decl (fn , name = "telegram_impl" , vis = "pub" , hash = "90c2aa78")] pub fn telegram_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n✉️ TELEGRAM! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
    };
}

telegram_impl!()