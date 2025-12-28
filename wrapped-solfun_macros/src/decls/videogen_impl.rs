macro_rules! videogen_impl {
    () => {
        # [decl (fn , name = "videogen_impl" , vis = "pub" , hash = "8a6bb12b")] pub fn videogen_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🎥 VIDEOGEN! Conceptually generating video about: \"{}\"\n" , # description) ; () } . into () }
    };
}

videogen_impl!();