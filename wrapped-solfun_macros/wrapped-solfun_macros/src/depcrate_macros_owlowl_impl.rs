// Generated macro for owl_impl (function)
macro_rules! Depcrate_macros_owlowl_impl {
() => {
// Module: crate::macros::owl
// Provides: {"owl_impl"}
// Dependencies: {}
# [decl (fn , name = "owl_impl" , vis = "pub" , hash = "6b285701")] pub fn owl_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🦉 OWL! Conceptual interaction: \"{{}}\"\n" , # description) ; () } . into () }
};
}
