macro_rules! owl_impl {
    () => {
        # [decl (fn , name = "owl_impl" , vis = "pub" , hash = "6b285701")] pub fn owl_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🦉 OWL! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
    };
}

owl_impl!();