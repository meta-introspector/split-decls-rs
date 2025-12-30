// Generated macro for binary_to_json_not_json (function)
macro_rules! Depcrate_data_testsbinary_to_json_not_json {
() => {
// Module: crate::data::tests
// Provides: {"binary_to_json_not_json"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn binary_to_json_not_json () { let binary = String :: from ("test") . into_bytes () ; let d = Data :: binary (binary) ; let d = d . coerce_to (DataFormat :: Json) ; assert_ne ! (DataFormat :: Json , d . format ()) ; assert_eq ! (DataFormat :: Binary , d . format ()) ; }
};
}
