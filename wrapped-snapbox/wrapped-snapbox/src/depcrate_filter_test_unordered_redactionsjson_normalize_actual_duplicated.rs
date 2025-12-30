// Generated macro for json_normalize_actual_duplicated (function)
macro_rules! Depcrate_filter_test_unordered_redactionsjson_normalize_actual_duplicated {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"json_normalize_actual_duplicated"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_actual_duplicated () { let input = json ! ([1 , 2 , 2 , 3]) ; let pattern = json ! ([1 , 2 , 3]) ; let expected = json ! ([1 , 2 , 3 , 2]) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (input) , & Data :: json (pattern)) ; assert_eq ! (actual , Data :: json (expected)) ; }
};
}
