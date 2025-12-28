macro_rules! github_impl {
    () => {
        # [decl (fn , name = "github_impl" , vis = "pub" , hash = "fe6f7b6e")] pub fn github_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🐙 GITHUB! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
    };
}

github_impl!()