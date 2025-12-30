// Generated macro for json_normalize_empty (function)
macro_rules! Depcrate_filter_test_unordered_redactionsjson_normalize_empty {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"json_normalize_empty"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_empty () { let input = json ! ([]) ; let pattern = json ! ([]) ; let expected = json ! ([]) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (input) , & Data :: json (pattern)) ; assert_eq ! (actual , Data :: json (expected)) ; }
};
}
