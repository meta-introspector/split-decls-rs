// Generated macro for text_to_binary (function)
macro_rules! Depcrate_data_teststext_to_binary {
() => {
// Module: crate::data::tests
// Provides: {"text_to_binary"}
// Dependencies: {}
# [test] fn text_to_binary () { let text = String :: from ("test") ; let d = Data :: text (text) ; let binary = d . coerce_to (DataFormat :: Binary) ; assert_eq ! (DataFormat :: Binary , binary . format ()) ; }
};
}
