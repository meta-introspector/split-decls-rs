// Generated macro for derive_deserialize (function)
macro_rules! Depcratederive_deserialize {
() => {
// Module: crate
// Provides: {"derive_deserialize"}
// Dependencies: {}
# [proc_macro_derive (Deserialize , attributes (serde))] pub fn derive_deserialize (input : TokenStream) -> TokenStream { let mut input = parse_macro_input ! (input as DeriveInput) ; de :: expand_derive_deserialize (& mut input) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
};
}
