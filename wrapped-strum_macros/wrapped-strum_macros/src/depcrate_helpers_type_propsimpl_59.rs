// Generated macro for impl_59 (impl)
macro_rules! Depcrate_helpers_type_propsimpl_59 {
() => {
// Module: crate::helpers::type_props
// Provides: {"impl_59"}
// Dependencies: {}
impl StrumTypeProperties { pub fn crate_module_path (& self) -> Path { self . crate_module_path . as_ref () . map_or_else (| | parse_quote ! (:: strum) , | path | parse_quote ! (# path)) } }
};
}
