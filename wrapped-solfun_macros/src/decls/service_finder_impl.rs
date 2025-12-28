macro_rules! service_finder_impl {
    () => {
        # [decl (fn , name = "service_finder_impl" , vis = "pub" , hash = "7dd619c4")] pub fn service_finder_impl (input : TokenStream) -> TokenStream { let need_description = parse_macro_input ! (input as LitStr) ; let span = need_description . span () ; let found_provider = format ! ("Found provider for need: \'{}\'" , need_description . value ()) ; quote_spanned ! { span => eprintln ! ("\n🔍 SERVICE FINDER! Searching for provider for: \'{}\''. Result: \'{}\'.\n" , # need_description , # found_provider) ; () } . into () }
    };
}

service_finder_impl!();