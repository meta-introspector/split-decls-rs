// Generated macro for text_to_bin_coerce_equals_to_bytes (function)
macro_rules! Depcrate_data_teststext_to_bin_coerce_equals_to_bytes {
() => {
// Module: crate::data::tests
// Provides: {"text_to_bin_coerce_equals_to_bytes"}
// Dependencies: {}
# [test] fn text_to_bin_coerce_equals_to_bytes () { let text = String :: from ("test") ; let d = Data :: text (text) ; let binary = d . clone () . coerce_to (DataFormat :: Binary) ; assert_eq ! (Data :: binary (d . to_bytes () . unwrap ()) , binary) ; }
};
}
