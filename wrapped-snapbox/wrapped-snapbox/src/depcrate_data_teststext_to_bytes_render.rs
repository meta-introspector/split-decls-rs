// Generated macro for text_to_bytes_render (function)
macro_rules! Depcrate_data_teststext_to_bytes_render {
() => {
// Module: crate::data::tests
// Provides: {"text_to_bytes_render"}
// Dependencies: {}
# [test] fn text_to_bytes_render () { let d = Data :: text (String :: from ("test")) ; let bytes = d . to_bytes () . unwrap () ; let bytes = String :: from_utf8 (bytes) . unwrap () ; let rendered = d . render () . unwrap () ; assert_eq ! (bytes , rendered) ; }
};
}
