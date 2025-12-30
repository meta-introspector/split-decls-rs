// Generated macro for str_normalize_post_elide_diverge (function)
macro_rules! Depcrate_filter_test_unordered_redactionsstr_normalize_post_elide_diverge {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"str_normalize_post_elide_diverge"}
// Dependencies: {}
# [test] fn str_normalize_post_elide_diverge () { let input = "Hello\nSun\nAnd\nWorld" ; let pattern = "Hello\n...\nMoon" ; let expected = "Hello\n...\n" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
