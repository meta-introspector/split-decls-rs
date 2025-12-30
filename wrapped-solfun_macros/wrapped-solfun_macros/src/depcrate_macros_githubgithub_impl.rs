// Generated macro for github_impl (function)
macro_rules! Depcrate_macros_githubgithub_impl {
() => {
// Module: crate::macros::github
// Provides: {"github_impl"}
// Dependencies: {}
# [decl (fn , name = "github_impl" , vis = "pub" , hash = "fe6f7b6e")] pub fn github_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🐙 GITHUB! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
};
}
