// Generated macro for json_normalize_reverse_order (function)
macro_rules! Depcrate_filter_test_unordered_redactionsjson_normalize_reverse_order {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"json_normalize_reverse_order"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_reverse_order () { let input = json ! ([1 , 2 , 3]) ; let pattern = json ! ([3 , 2 , 1]) ; let expected = json ! ([3 , 2 , 1]) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (input) , & Data :: json (pattern)) ; assert_eq ! (actual , Data :: json (expected)) ; }
};
}
