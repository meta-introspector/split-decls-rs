// Generated macro for json_normalize_glob_for_obj (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_glob_for_obj {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_glob_for_obj"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_glob_for_obj () { let exp = json ! ({ "people" : "{...}" }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : { "name" : "JohnDoe" , "nickname" : "John" , } }) ; let actual = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
};
}
