// Generated macro for derive_json_schema_repr_wrapper (function)
macro_rules! Depcratederive_json_schema_repr_wrapper {
() => {
// Module: crate
// Provides: {"derive_json_schema_repr_wrapper"}
// Dependencies: {}
# [proc_macro_derive (JsonSchema_repr , attributes (schemars , serde))] pub fn derive_json_schema_repr_wrapper (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as syn :: DeriveInput) ; derive_json_schema (input , true) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
};
}
