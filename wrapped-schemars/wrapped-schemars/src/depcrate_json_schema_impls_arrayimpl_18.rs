// Generated macro for impl_18 (impl)
macro_rules! Depcrate_json_schema_impls_arrayimpl_18 {
() => {
// Module: crate::json_schema_impls::array
// Provides: {"impl_18"}
// Dependencies: {}
impl < T > JsonSchema for [T ; 0] { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "EmptyArray" . into () } fn schema_id () -> Cow < 'static , str > { "[]" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "maxItems" : 0 , }) } }
};
}
