// Generated macro for json_normalize_glob_for_array_middle_end (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_glob_for_array_middle_end {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_glob_for_array_middle_end"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_glob_for_array_middle_end () { let exp = json ! ([{ "name" : "one" , "nickname" : "1" , } , "{...}" , { "name" : "three" , "nickname" : "3" , } , "{...}"]) ; let expected = Data :: json (exp) ; let actual = json ! ([{ "name" : "one" , "nickname" : "1" , } , { "name" : "two" , "nickname" : "2" , } , { "name" : "three" , "nickname" : "3" , } , { "name" : "four" , "nickname" : "4" , } , { "name" : "five" , "nickname" : "5" , }]) ; let actual = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
};
}
