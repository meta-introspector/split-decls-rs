// Generated macro for replace_version_impl (function)
macro_rules! Depcrate_macros_replace_versionreplace_version_impl {
() => {
// Module: crate::macros::replace_version
// Provides: {"replace_version_impl"}
// Dependencies: {}
# [decl (fn , name = "replace_version_impl" , vis = "pub" , hash = "e5280982")] pub fn replace_version_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n⬆️ REPLACE VERSION! Action: {}\n" , # description) ; () } . into () }
};
}
