macro_rules! bug_impl {
    () => {
        # [decl (fn , name = "bug_impl" , vis = "pub" , hash = "cc2ff72d")] pub fn bug_impl (input : TokenStream) -> TokenStream { let bug_description = parse_macro_input ! (input as LitStr) ; let span = bug_description . span () ; quote_spanned ! { span => eprintln ! ("\n🐛 BUG! Reported bug: \"{}\"\n" , # bug_description) ; () } . into () }
    };
}

bug_impl!();