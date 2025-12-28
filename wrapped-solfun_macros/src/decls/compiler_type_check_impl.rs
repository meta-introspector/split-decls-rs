macro_rules! compiler_type_check_impl {
    () => {
        # [decl (fn , name = "compiler_type_check_impl" , vis = "pub" , hash = "eb0a188f")] pub fn compiler_type_check_impl (input : TokenStream) -> TokenStream { let code_snippet = parse_macro_input ! (input as LitStr) ; let span = code_snippet . span () ; quote_spanned ! { span => eprintln ! ("\n✅ COMPILER TYPE CHECK! Checking type of: {}\n" , # code_snippet) ; () } . into () }
    };
}

compiler_type_check_impl!()