// Generated macro for impl_119 (impl)
macro_rules! Depcrate_json_schema_impls_std_timeimpl_119 {
() => {
// Module: crate::json_schema_impls::std_time
// Provides: {"impl_119"}
// Dependencies: {}
impl JsonSchema for core :: time :: Duration { fn schema_name () -> Cow < 'static , str > { "Duration" . into () } fn schema_id () -> Cow < 'static , str > { "std::time::Duration" . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "object" , "required" : ["secs" , "nanos"] , "properties" : { "secs" : u64 :: json_schema (generator) , "nanos" : u32 :: json_schema (generator) , } }) } }
};
}
