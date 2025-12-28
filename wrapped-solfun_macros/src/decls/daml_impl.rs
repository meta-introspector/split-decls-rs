macro_rules! daml_impl {
    () => {
        # [decl (fn , name = "daml_impl" , vis = "pub" , hash = "4d27600f")] pub fn daml_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n📜 DAML! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
    };
}

daml_impl!()