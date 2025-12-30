// Generated macro for str_normalize_empty (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_empty {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_empty"}
// Dependencies: {}
# [test] fn str_normalize_empty () { let input = "" ; let pattern = "" ; let expected = "" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
