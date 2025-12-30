// Generated macro for json_normalize_glob_array_start (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_glob_array_start {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_glob_array_start"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_glob_array_start () { let exp = json ! ({ "people" : ["{...}" , { "name" : "three" , "nickname" : "3" , }] }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : [{ "name" : "one" , "nickname" : "1" , } , { "name" : "two" , "nickname" : "2" , } , { "name" : "three" , "nickname" : "3" , }] }) ; let actual = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
};
}
