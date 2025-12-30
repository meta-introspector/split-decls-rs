// Generated macro for impl_31 (impl)
macro_rules! Depcrate_json_schema_impls_coreimpl_31 {
() => {
// Module: crate::json_schema_impls::core
// Provides: {"impl_31"}
// Dependencies: {}
impl < T : JsonSchema > JsonSchema for Range < T > { fn schema_name () -> Cow < 'static , str > { format ! ("Range_of_{}" , T :: schema_name ()) . into () } fn schema_id () -> Cow < 'static , str > { format ! ("Range<{}>" , T :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { let subschema = generator . subschema_for :: < T > () ; json_schema ! ({ "type" : "object" , "properties" : { "start" : subschema , "end" : subschema } , "required" : ["start" , "end"] }) } }
};
}
