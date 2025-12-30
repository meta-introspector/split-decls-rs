// Generated macro for str_normalize_leading_elide (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_leading_elide {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_leading_elide"}
// Dependencies: {}
# [test] fn str_normalize_leading_elide () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "...\nGoodbye" ; let expected = "...\nGoodbye" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
