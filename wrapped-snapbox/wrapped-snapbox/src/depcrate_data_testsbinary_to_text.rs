// Generated macro for binary_to_text (function)
macro_rules! Depcrate_data_testsbinary_to_text {
() => {
// Module: crate::data::tests
// Provides: {"binary_to_text"}
// Dependencies: {}
# [test] fn binary_to_text () { let binary = String :: from ("test") . into_bytes () ; let d = Data :: binary (binary) ; let text = d . coerce_to (DataFormat :: Text) ; assert_eq ! (DataFormat :: Text , text . format ()) ; }
};
}
