// Generated macro for impl_169 (impl)
macro_rules! Depcrate_json_schema_impls_ffiimpl_169 {
() => {
// Module: crate::json_schema_impls::ffi
// Provides: {"impl_169"}
// Dependencies: {}
impl JsonSchema for CString { fn schema_name () -> Cow < 'static , str > { "CString" . into () } fn schema_id () -> Cow < 'static , str > { "std::ffi::CString" . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { let ty = if generator . contract () . is_deserialize () { json ! (["array" , "string"]) } else { json ! ("array") } ; json_schema ! ({ "type" : ty , "items" : { "type" : "integer" , "minimum" : 1 , "maximum" : 255 } , }) } }
};
}
