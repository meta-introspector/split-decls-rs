// Generated macro for json_to_bin_coerce_equals_to_bytes (function)
macro_rules! Depcrate_data_testsjson_to_bin_coerce_equals_to_bytes {
() => {
// Module: crate::data::tests
// Provides: {"json_to_bin_coerce_equals_to_bytes"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_to_bin_coerce_equals_to_bytes () { let json = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (json) ; let binary = d . clone () . coerce_to (DataFormat :: Binary) ; assert_eq ! (Data :: binary (d . to_bytes () . unwrap ()) , binary) ; }
};
}
