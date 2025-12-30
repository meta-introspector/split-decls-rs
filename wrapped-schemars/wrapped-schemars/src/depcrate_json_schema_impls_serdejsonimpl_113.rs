// Generated macro for impl_113 (impl)
macro_rules! Depcrate_json_schema_impls_serdejsonimpl_113 {
() => {
// Module: crate::json_schema_impls::serdejson
// Provides: {"impl_113"}
// Dependencies: {}
impl JsonSchema for Number { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "Number" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "number" }) } }
};
}
