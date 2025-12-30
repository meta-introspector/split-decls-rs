// Generated macro for derive_json_schema_wrapper (function)
macro_rules! Depcratederive_json_schema_wrapper {
() => {
// Module: crate
// Provides: {"derive_json_schema_wrapper"}
// Dependencies: {}
# [doc = "Derive macro for `JsonSchema` trait."] # [cfg_attr (not (doctest) , allow (clippy :: needless_doctest_main) , doc = include_str ! ("../deriving.md") , doc = include_str ! ("../attributes.md"))] # [proc_macro_derive (JsonSchema , attributes (schemars , serde , validate , garde))] pub fn derive_json_schema_wrapper (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as syn :: DeriveInput) ; derive_json_schema (input , false) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
};
}
