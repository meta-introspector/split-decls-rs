// Generated macro for impl_120 (impl)
macro_rules! Depcrate_json_schema_impls_std_timeimpl_120 {
() => {
// Module: crate::json_schema_impls::std_time
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "std")] impl JsonSchema for std :: time :: SystemTime { fn schema_name () -> Cow < 'static , str > { "SystemTime" . into () } fn schema_id () -> Cow < 'static , str > { "std::time::SystemTime" . into () } fn json_schema (generator : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "object" , "required" : ["secs_since_epoch" , "nanos_since_epoch"] , "properties" : { "secs_since_epoch" : u64 :: json_schema (generator) , "nanos_since_epoch" : u32 :: json_schema (generator) , } }) } }
};
}
