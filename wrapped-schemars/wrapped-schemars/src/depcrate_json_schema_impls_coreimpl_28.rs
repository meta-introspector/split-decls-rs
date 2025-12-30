// Generated macro for impl_28 (impl)
macro_rules! Depcrate_json_schema_impls_coreimpl_28 {
() => {
// Module: crate::json_schema_impls::core
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : JsonSchema > JsonSchema for Option < T > { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { format ! ("Nullable_{}" , T :: schema_name ()) . into () } fn schema_id () -> Cow < 'static , str > { format ! ("Option<{}>" , T :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { let mut schema = generator . subschema_for :: < T > () ; allow_null (generator , & mut schema) ; schema } fn _schemars_private_non_optional_json_schema (generator : & mut SchemaGenerator) -> Schema { T :: _schemars_private_non_optional_json_schema (generator) } fn _schemars_private_is_option () -> bool { true } }
};
}
