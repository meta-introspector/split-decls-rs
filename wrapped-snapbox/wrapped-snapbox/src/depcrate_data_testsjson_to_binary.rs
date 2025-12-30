// Generated macro for json_to_binary (function)
macro_rules! Depcrate_data_testsjson_to_binary {
() => {
// Module: crate::data::tests
// Provides: {"json_to_binary"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_to_binary () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (value) ; let binary = d . coerce_to (DataFormat :: Binary) ; assert_eq ! (DataFormat :: Binary , binary . format ()) ; }
};
}
