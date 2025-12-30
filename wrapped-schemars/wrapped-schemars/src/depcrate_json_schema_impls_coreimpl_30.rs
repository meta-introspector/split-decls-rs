// Generated macro for impl_30 (impl)
macro_rules! Depcrate_json_schema_impls_coreimpl_30 {
() => {
// Module: crate::json_schema_impls::core
// Provides: {"impl_30"}
// Dependencies: {}
impl < T : JsonSchema > JsonSchema for Bound < T > { fn schema_name () -> Cow < 'static , str > { format ! ("Bound_of_{}" , T :: schema_name ()) . into () } fn schema_id () -> Cow < 'static , str > { format ! ("Bound<{}>" , T :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "oneOf" : [{ "type" : "object" , "properties" : { "Included" : generator . subschema_for ::< T > () } , "required" : ["Included"] } , { "type" : "object" , "properties" : { "Excluded" : generator . subschema_for ::< T > () } , "required" : ["Excluded"] } , { "type" : "string" , "const" : "Unbounded" }] }) } }
};
}
