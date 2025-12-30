// Generated macro for binary_to_text_not_utf8 (function)
macro_rules! Depcrate_data_testsbinary_to_text_not_utf8 {
() => {
// Module: crate::data::tests
// Provides: {"binary_to_text_not_utf8"}
// Dependencies: {}
# [test] fn binary_to_text_not_utf8 () { let binary = b"\xFF\xE0\x00\x10\x4A\x46\x49\x46\x00" . to_vec () ; let d = Data :: binary (binary) ; let d = d . coerce_to (DataFormat :: Text) ; assert_ne ! (DataFormat :: Text , d . format ()) ; assert_eq ! (DataFormat :: Binary , d . format ()) ; }
};
}
