// Generated macro for str_normalize_input_shorter (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_input_shorter {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_input_shorter"}
// Dependencies: {}
# [test] fn str_normalize_input_shorter () { let input = "Hello\n" ; let pattern = "Hello\nWorld" ; let expected = "Hello\n" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
