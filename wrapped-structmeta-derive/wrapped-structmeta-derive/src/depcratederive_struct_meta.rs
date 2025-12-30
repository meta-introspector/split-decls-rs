// Generated macro for derive_struct_meta (function)
macro_rules! Depcratederive_struct_meta {
() => {
// Module: crate
// Provides: {"derive_struct_meta"}
// Dependencies: {}
# [proc_macro_derive (StructMeta , attributes (struct_meta))] pub fn derive_struct_meta (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; into_macro_output (struct_meta :: derive_struct_meta (input)) }
};
}
