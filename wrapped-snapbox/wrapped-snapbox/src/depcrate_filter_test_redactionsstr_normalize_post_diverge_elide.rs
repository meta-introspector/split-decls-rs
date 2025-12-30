// Generated macro for str_normalize_post_diverge_elide (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_post_diverge_elide {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_post_diverge_elide"}
// Dependencies: {}
# [test] fn str_normalize_post_diverge_elide () { let input = "Hello\nWorld\nGoodbye\nSir" ; let pattern = "Hello\nMoon\nGoodbye\n..." ; let expected = "Hello\nWorld\nGoodbye\n..." ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
