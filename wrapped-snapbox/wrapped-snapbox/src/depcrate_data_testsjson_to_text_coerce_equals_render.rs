// Generated macro for json_to_text_coerce_equals_render (function)
macro_rules! Depcrate_data_testsjson_to_text_coerce_equals_render {
() => {
// Module: crate::data::tests
// Provides: {"json_to_text_coerce_equals_render"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_to_text_coerce_equals_render () { let json = json ! ({ "name" : "John\\Doe\r\n" }) ; let d = Data :: json (json) ; let text = d . clone () . coerce_to (DataFormat :: Text) ; assert_eq ! (Data :: text (d . render () . unwrap ()) , text) ; }
};
}
