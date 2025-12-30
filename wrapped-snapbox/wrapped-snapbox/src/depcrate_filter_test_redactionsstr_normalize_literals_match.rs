// Generated macro for str_normalize_literals_match (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_literals_match {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_literals_match"}
// Dependencies: {}
# [test] fn str_normalize_literals_match () { let input = "Hello\nWorld" ; let pattern = "Hello\nWorld" ; let expected = "Hello\nWorld" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
