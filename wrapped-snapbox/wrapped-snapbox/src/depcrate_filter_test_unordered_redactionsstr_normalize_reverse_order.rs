// Generated macro for str_normalize_reverse_order (function)
macro_rules! Depcrate_filter_test_unordered_redactionsstr_normalize_reverse_order {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"str_normalize_reverse_order"}
// Dependencies: {}
# [test] fn str_normalize_reverse_order () { let input = "1
2
3
" ; let pattern = "3
2
1
" ; let expected = "3
2
1
" ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (input . into_data () , & pattern . into_data ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
