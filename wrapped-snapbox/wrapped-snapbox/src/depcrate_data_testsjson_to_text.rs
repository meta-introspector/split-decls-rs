// Generated macro for json_to_text (function)
macro_rules! Depcrate_data_testsjson_to_text {
() => {
// Module: crate::data::tests
// Provides: {"json_to_text"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_to_text () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (value) ; let text = d . coerce_to (DataFormat :: Text) ; assert_eq ! (DataFormat :: Text , text . format ()) ; }
};
}
