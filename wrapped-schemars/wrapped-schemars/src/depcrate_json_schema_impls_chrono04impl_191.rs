// Generated macro for impl_191 (impl)
macro_rules! Depcrate_json_schema_impls_chrono04impl_191 {
() => {
// Module: crate::json_schema_impls::chrono04
// Provides: {"impl_191"}
// Dependencies: {}
impl JsonSchema for Weekday { inline_schema ! () ; fn schema_name () -> Cow < 'static , str > { "Weekday" . into () } fn schema_id () -> Cow < 'static , str > { "chrono::Weekday" . into () } fn json_schema (_ : & mut SchemaGenerator) -> Schema { json_schema ! ({ "type" : "string" , "enum" : ["Mon" , "Tue" , "Wed" , "Thu" , "Fri" , "Sat" , "Sun" ,] }) } }
};
}
