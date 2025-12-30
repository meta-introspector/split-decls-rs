// Generated macro for videogen_impl (function)
macro_rules! Depcrate_macros_videogenvideogen_impl {
() => {
// Module: crate::macros::videogen
// Provides: {"videogen_impl"}
// Dependencies: {}
# [decl (fn , name = "videogen_impl" , vis = "pub" , hash = "8a6bb12b")] pub fn videogen_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🎥 VIDEOGEN! Conceptually generating video about: \"{}\"\n" , # description) ; () } . into () }
};
}
