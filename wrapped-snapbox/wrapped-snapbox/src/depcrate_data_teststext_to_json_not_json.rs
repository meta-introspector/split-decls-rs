// Generated macro for text_to_json_not_json (function)
macro_rules! Depcrate_data_teststext_to_json_not_json {
() => {
// Module: crate::data::tests
// Provides: {"text_to_json_not_json"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn text_to_json_not_json () { let text = String :: from ("test") ; let d = Data :: text (text) ; let json = d . coerce_to (DataFormat :: Json) ; assert_eq ! (DataFormat :: Text , json . format ()) ; }
};
}
