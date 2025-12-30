// Generated macro for str_normalize_all_different (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_all_different {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_all_different"}
// Dependencies: {}
# [test] fn str_normalize_all_different () { let input = "Hello\nWorld" ; let pattern = "Goodbye\nMoon" ; let expected = "Hello\nWorld" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
