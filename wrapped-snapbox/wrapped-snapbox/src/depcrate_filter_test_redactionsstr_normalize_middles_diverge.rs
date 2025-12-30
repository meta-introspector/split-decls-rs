// Generated macro for str_normalize_middles_diverge (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_middles_diverge {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_middles_diverge"}
// Dependencies: {}
# [test] fn str_normalize_middles_diverge () { let input = "Hello\nWorld\nGoodbye" ; let pattern = "Hello\nMoon\nGoodbye" ; let expected = "Hello\nWorld\nGoodbye" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
