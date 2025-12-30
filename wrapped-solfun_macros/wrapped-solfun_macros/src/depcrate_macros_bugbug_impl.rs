// Generated macro for bug_impl (function)
macro_rules! Depcrate_macros_bugbug_impl {
() => {
// Module: crate::macros::bug
// Provides: {"bug_impl"}
// Dependencies: {}
# [decl (fn , name = "bug_impl" , vis = "pub" , hash = "cc2ff72d")] pub fn bug_impl (input : TokenStream) -> TokenStream { let bug_description = parse_macro_input ! (input as LitStr) ; let span = bug_description . span () ; quote_spanned ! { span => eprintln ! ("\n🐛 BUG! Reported bug: \"{}\"\n" , # bug_description) ; () } . into () }
};
}
