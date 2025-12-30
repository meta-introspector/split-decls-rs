// Generated macro for text_to_json (function)
macro_rules! Depcrate_data_teststext_to_json {
() => {
// Module: crate::data::tests
// Provides: {"text_to_json"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn text_to_json () { let value = json ! ({ "name" : "John\\Doe\r\n" }) ; let text = serde_json :: to_string_pretty (& value) . unwrap () ; let d = Data :: text (text) ; let json = d . coerce_to (DataFormat :: Json) ; assert_eq ! (DataFormat :: Json , json . format ()) ; }
};
}
