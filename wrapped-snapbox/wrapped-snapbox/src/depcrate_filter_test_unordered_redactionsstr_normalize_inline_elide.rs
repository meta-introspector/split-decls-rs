// Generated macro for str_normalize_inline_elide (function)
macro_rules! Depcrate_filter_test_unordered_redactionsstr_normalize_inline_elide {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"str_normalize_inline_elide"}
// Dependencies: {}
# [test] fn str_normalize_inline_elide () { let input = "Hello\nWorld\nGoodbye\nSir" ; let pattern = "Hello\nW[..]d\nGoodbye\nSir" ; let expected = "Hello\nW[..]d\nGoodbye\nSir" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
