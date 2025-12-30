// Generated macro for oil_impl (function)
macro_rules! Depcrate_macros_oiloil_impl {
() => {
// Module: crate::macros::oil
// Provides: {"oil_impl"}
// Dependencies: {}
# [decl (fn , name = "oil_impl" , vis = "pub" , hash = "ea1d6293")] pub fn oil_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🛢️ OIL! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
};
}
