// Generated macro for impl_246 (impl)
macro_rules! Depcrate_json_schema_impls_url2impl_246 {
() => {
// Module: crate::json_schema_impls::url2
// Provides: {"impl_246"}
// Dependencies: {}
impl JsonSchema for Url { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "Url" . into () } fn schema_id () -> Cow < 'static , str > { "url::Url" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "string" , "format" : "uri" , }) } }
};
}
