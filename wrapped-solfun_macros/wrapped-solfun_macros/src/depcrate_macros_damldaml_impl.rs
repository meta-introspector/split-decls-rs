// Generated macro for daml_impl (function)
macro_rules! Depcrate_macros_damldaml_impl {
() => {
// Module: crate::macros::daml
// Provides: {"daml_impl"}
// Dependencies: {}
# [decl (fn , name = "daml_impl" , vis = "pub" , hash = "4d27600f")] pub fn daml_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n📜 DAML! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
};
}
