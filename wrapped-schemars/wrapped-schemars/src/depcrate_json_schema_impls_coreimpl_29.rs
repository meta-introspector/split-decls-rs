// Generated macro for impl_29 (impl)
macro_rules! Depcrate_json_schema_impls_coreimpl_29 {
() => {
// Module: crate::json_schema_impls::core
// Provides: {"impl_29"}
// Dependencies: {}
impl < T : JsonSchema , E : JsonSchema > JsonSchema for Result < T , E > { fn schema_name () -> Cow < 'static , str > { format ! ("Result_of_{}_or_{}" , T :: schema_name () , E :: schema_name ()) . into () } fn schema_id () -> Cow < 'static , str > { format ! ("Result<{}, {}>" , T :: schema_id () , E :: schema_id ()) . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "oneOf" : [{ "type" : "object" , "properties" : { "Ok" : generator . subschema_for ::< T > () } , "required" : ["Ok"] } , { "type" : "object" , "properties" : { "Err" : generator . subschema_for ::< E > () } , "required" : ["Err"] }] }) } }
};
}
