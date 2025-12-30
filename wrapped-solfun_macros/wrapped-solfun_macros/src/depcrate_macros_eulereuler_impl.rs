// Generated macro for euler_impl (function)
macro_rules! Depcrate_macros_eulereuler_impl {
() => {
// Module: crate::macros::euler
// Provides: {"euler_impl"}
// Dependencies: {}
# [decl (fn , name = "euler_impl" , vis = "pub" , hash = "b26f3bf9")] pub fn euler_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🗺️ EULER! Conceptual interaction: \"{}\"\n" , # description) ; () } . into () }
};
}
