// Generated macro for json_normalize_expected_missing (function)
macro_rules! Depcrate_filter_test_unordered_redactionsjson_normalize_expected_missing {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"json_normalize_expected_missing"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_expected_missing () { let input = json ! ([1 , 2 , 3]) ; let pattern = json ! ([1 , 3]) ; let expected = json ! ([1 , 3 , 2]) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (input) , & Data :: json (pattern)) ; assert_eq ! (actual , Data :: json (expected)) ; }
};
}
