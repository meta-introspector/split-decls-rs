// Generated macro for str_normalize_expected_missing (function)
macro_rules! Depcrate_filter_test_unordered_redactionsstr_normalize_expected_missing {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"str_normalize_expected_missing"}
// Dependencies: {}
# [test] fn str_normalize_expected_missing () { let input = "1
2
3
" ; let pattern = "1
3
" ; let expected = "1
3
2
" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into_data () , & pattern . into_data ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
