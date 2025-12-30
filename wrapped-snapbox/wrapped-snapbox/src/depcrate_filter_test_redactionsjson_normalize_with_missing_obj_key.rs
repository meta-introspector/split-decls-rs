// Generated macro for json_normalize_with_missing_obj_key (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_with_missing_obj_key {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_with_missing_obj_key"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_with_missing_obj_key () { let expected = json ! ({ "a" : "[A]" , "b" : "[B]" , "c" : "[C]" , }) ; let expected = Data :: json (expected) ; let actual = json ! ({ "a" : "value-a" , "c" : "value-c" , }) ; let actual = Data :: json (actual) ; let mut sub = Redactions :: new () ; sub . insert ("[A]" , "value-a") . unwrap () ; sub . insert ("[B]" , "value-b") . unwrap () ; sub . insert ("[C]" , "value-c") . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . normalize (actual , & expected) ; let expected_actual = json ! ({ "a" : "[A]" , "c" : "[C]" , }) ; let expected_actual = Data :: json (expected_actual) ; assert_eq ! (actual , expected_actual) ; }
};
}
