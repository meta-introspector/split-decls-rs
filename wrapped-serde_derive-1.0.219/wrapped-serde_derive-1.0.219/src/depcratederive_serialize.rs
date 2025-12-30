// Generated macro for derive_serialize (function)
macro_rules! Depcratederive_serialize {
() => {
// Module: crate
// Provides: {"derive_serialize"}
// Dependencies: {}
# [proc_macro_derive (Serialize , attributes (serde))] pub fn derive_serialize (input : TokenStream) -> TokenStream { let mut input = parse_macro_input ! (input as DeriveInput) ; ser :: expand_derive_serialize (& mut input) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
};
}
