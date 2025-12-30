// Generated macro for foaf_impl (function)
macro_rules! Depcrate_macros_foaffoaf_impl {
() => {
// Module: crate::macros::foaf
// Provides: {"foaf_impl"}
// Dependencies: {}
# [decl (fn , name = "foaf_impl" , vis = "pub" , hash = "b71ebb67")] pub fn foaf_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🧑‍🤝‍🧑 FOAF! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
};
}
