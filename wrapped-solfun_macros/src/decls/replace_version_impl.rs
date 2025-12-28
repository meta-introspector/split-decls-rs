macro_rules! replace_version_impl {
    () => {
        # [decl (fn , name = "replace_version_impl" , vis = "pub" , hash = "e5280982")] pub fn replace_version_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n⬆️ REPLACE VERSION! Action: {}\n" , # description) ; () } . into () }
    };
}

replace_version_impl!();