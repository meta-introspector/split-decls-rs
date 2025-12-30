// Generated macro for impl_192 (impl)
macro_rules! Depcrate_json_schema_impls_chrono04impl_192 {
() => {
// Module: crate::json_schema_impls::chrono04
// Provides: {"impl_192"}
// Dependencies: {}
impl JsonSchema for TimeDelta { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "TimeDelta" . into () } fn schema_id () -> Cow < 'static , str > { "chrono::TimeDelta" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "array" , "prefixItems" : [{ "type" : "integer" , "format" : "int64" } , { "type" : "integer" , "minimum" : 0 , "exclusiveMaximum" : 1_000_000_000 }] , "minItems" : 2 , "maxItems" : 2 }) } }
};
}
