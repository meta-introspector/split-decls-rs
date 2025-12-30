// Generated macro for json_to_bytes_render (function)
macro_rules! Depcrate_data_testsjson_to_bytes_render {
() => {
// Module: crate::data::tests
// Provides: {"json_to_bytes_render"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_to_bytes_render () { let d = Data :: json (json ! ({ "name" : "John\\Doe\r\n" })) ; let bytes = d . to_bytes () . unwrap () ; let bytes = String :: from_utf8 (bytes) . unwrap () ; let rendered = d . render () . unwrap () ; assert_eq ! (bytes , rendered) ; }
};
}
