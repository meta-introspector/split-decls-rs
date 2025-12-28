macro_rules! meme_impl {
    () => {
        # [decl (fn , name = "meme_impl" , vis = "pub" , hash = "d433e7fb")] pub fn meme_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🐸 MEME! Generating/displaying meme about: \"{}\"\n" , # description) ; () } . into () }
    };
}

meme_impl!()