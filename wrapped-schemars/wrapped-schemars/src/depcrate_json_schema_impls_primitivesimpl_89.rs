// Generated macro for impl_89 (impl)
macro_rules! Depcrate_json_schema_impls_primitivesimpl_89 {
() => {
// Module: crate::json_schema_impls::primitives
// Provides: {"impl_89"}
// Dependencies: {}
impl JsonSchema for char { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "Character" . into () } fn schema_id () -> Cow < 'static , str > { "char" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "string" , "minLength" : 1 , "maxLength" : 1 , }) } }
};
}
