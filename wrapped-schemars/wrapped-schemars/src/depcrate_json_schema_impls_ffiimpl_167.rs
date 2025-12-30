// Generated macro for impl_167 (impl)
macro_rules! Depcrate_json_schema_impls_ffiimpl_167 {
() => {
// Module: crate::json_schema_impls::ffi
// Provides: {"impl_167"}
// Dependencies: {}
impl JsonSchema for OsString { fn schema_name () -> Cow < 'static , str > { "OsString" . into () } fn schema_id () -> Cow < 'static , str > { "std::ffi::OsString" . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "oneOf" : [{ "type" : "object" , "properties" : { "Unix" : < Vec < u8 >>:: json_schema (generator) } , "required" : ["Unix"] } , { "type" : "object" , "properties" : { "Windows" : < Vec < u16 >>:: json_schema (generator) } , "required" : ["Windows"] } ,] }) } }
};
}
