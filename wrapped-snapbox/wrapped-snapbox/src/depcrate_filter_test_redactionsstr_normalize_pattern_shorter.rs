// Generated macro for str_normalize_pattern_shorter (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_pattern_shorter {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_pattern_shorter"}
// Dependencies: {}
# [test] fn str_normalize_pattern_shorter () { let input = "Hello\nWorld" ; let pattern = "Hello\n" ; let expected = "Hello\nWorld" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
