macro_rules! compiler_parser_element_impl {
    () => {
        # [decl (fn , name = "compiler_parser_element_impl" , vis = "pub" , hash = "0eef2414")] pub fn compiler_parser_element_impl (input : TokenStream) -> TokenStream { let element_name = parse_macro_input ! (input as LitStr) ; let span = element_name . span () ; quote_spanned ! { span => eprintln ! ("\n🔍 COMPILER PARSER ELEMENT! Interacting with: {}\n" , # element_name) ; () } . into () }
    };
}

compiler_parser_element_impl!()