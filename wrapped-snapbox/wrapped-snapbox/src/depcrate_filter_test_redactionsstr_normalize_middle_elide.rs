// Generated macro for str_normalize_middle_elide (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_middle_elide {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_middle_elide"}
// Dependencies: {}
# [test] fn str_normalize_middle_elide () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "Hello\n...\nGoodbye" ; let expected = "Hello\n...\nGoodbye" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
