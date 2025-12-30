// Generated macro for json_normalize_obj_key (function)
macro_rules! Depcrate_filter_test_unordered_redactionsjson_normalize_obj_key {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"json_normalize_obj_key"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_obj_key () { let expected = json ! ({ "[A]" : "value-a" , "[B]" : "value-b" , "[C]" : "value-c" , }) ; let expected = Data :: json (expected) ; let actual = json ! ({ "key-a" : "value-a" , "key-b" : "value-b" , "key-c" : "value-c" , }) ; let actual = Data :: json (actual) ; let mut sub = Redactions :: new () ; sub . insert ("[A]" , "key-a") . unwrap () ; sub . insert ("[B]" , "key-b") . unwrap () ; sub . insert ("[C]" , "key-c") . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . unordered () . normalize (actual , & expected) ; let expected_actual = json ! ({ "[A]" : "value-a" , "[B]" : "value-b" , "[C]" : "value-c" , }) ; let expected_actual = Data :: json (expected_actual) ; assert_eq ! (actual , expected_actual) ; }
};
}
