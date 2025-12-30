// Generated macro for binary_to_json (function)
macro_rules! Depcrate_data_testsbinary_to_json {
() => {
// Module: crate::data::tests
// Provides: {"binary_to_json"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn binary_to_json () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let binary = serde_json :: to_vec_pretty (& value) . unwrap () ; let d = Data :: binary (binary) ; let json = d . coerce_to (DataFormat :: Json) ; assert_eq ! (DataFormat :: Json , json . format ()) ; }
};
}
