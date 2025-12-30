// Generated macro for derive_my_msg (function)
macro_rules! Depcratederive_my_msg {
() => {
// Module: crate
// Provides: {"derive_my_msg"}
// Dependencies: {}
# [proc_macro_derive (MyMsg , attributes (my_msg))] pub fn derive_my_msg (input : TokenStream) -> TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; let mut msg = String :: new () ; for attr in input . attrs { if attr . path () . is_ident ("my_msg") { let attr = attr . parse_args :: < MyAttr > () . unwrap () ; msg = attr . msg . value () ; } } quote ! (const MSG : & str = # msg ;) . into () }
};
}
