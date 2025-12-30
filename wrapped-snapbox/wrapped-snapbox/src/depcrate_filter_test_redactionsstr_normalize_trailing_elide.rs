// Generated macro for str_normalize_trailing_elide (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_trailing_elide {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_trailing_elide"}
// Dependencies: {}
# [test] fn str_normalize_trailing_elide () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "Hello\n..." ; let expected = "Hello\n..." ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
