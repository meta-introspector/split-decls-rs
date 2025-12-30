// Generated macro for json_normalize_glob_obj_key (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_glob_obj_key {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_glob_obj_key"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_glob_obj_key () { let expected = json ! ({ "a" : "value-a" , "c" : "value-c" , "..." : "{...}" , }) ; let expected = Data :: json (expected) ; let actual = json ! ({ "a" : "value-a" , "b" : "value-b" , "c" : "value-c" , }) ; let actual = Data :: json (actual) ; let actual = NormalizeToExpected :: new () . redact () . normalize (actual , & expected) ; let expected_actual = json ! ({ "a" : "value-a" , "c" : "value-c" , "..." : "{...}" , }) ; let expected_actual = Data :: json (expected_actual) ; assert_eq ! (actual , expected_actual) ; }
};
}
