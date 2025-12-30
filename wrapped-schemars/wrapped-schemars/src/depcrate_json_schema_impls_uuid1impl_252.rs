// Generated macro for impl_252 (impl)
macro_rules! Depcrate_json_schema_impls_uuid1impl_252 {
() => {
// Module: crate::json_schema_impls::uuid1
// Provides: {"impl_252"}
// Dependencies: {}
impl JsonSchema for Uuid { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "Uuid" . into () } fn schema_id () -> Cow < 'static , str > { "uuid::Uuid" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "string" , "format" : "uuid" , }) } }
};
}
