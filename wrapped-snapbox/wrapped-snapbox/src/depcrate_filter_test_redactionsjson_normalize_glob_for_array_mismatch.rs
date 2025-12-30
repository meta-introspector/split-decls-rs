// Generated macro for json_normalize_glob_for_array_mismatch (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_glob_for_array_mismatch {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_glob_for_array_mismatch"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_glob_for_array_mismatch () { let exp = json ! ([{ "name" : "one" , "nickname" : "1" , } , "{...}" , { "name" : "three" , "nickname" : "3" , } , "{...}"]) ; let expected = Data :: json (exp) ; let actual = json ! ([{ "name" : "one" , "nickname" : "1" , } , { "name" : "two" , "nickname" : "2" , } , { "name" : "four" , "nickname" : "4" , } , { "name" : "five" , "nickname" : "5" , }]) ; let actual_normalized = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual . clone ()) , & expected) ; if let DataInner :: Json (act) = actual_normalized . inner { assert_eq ! (act , actual) ; } }
};
}
