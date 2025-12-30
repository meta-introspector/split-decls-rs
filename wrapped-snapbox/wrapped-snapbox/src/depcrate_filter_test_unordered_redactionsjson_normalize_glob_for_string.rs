// Generated macro for json_normalize_glob_for_string (function)
macro_rules! Depcrate_filter_test_unordered_redactionsjson_normalize_glob_for_string {
() => {
// Module: crate::filter::test_unordered_redactions
// Provides: {"json_normalize_glob_for_string"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_glob_for_string () { let exp = json ! ({ "name" : "{...}" }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "name" : "JohnDoe" }) ; let actual = NormalizeToExpected :: new () . redact () . unordered () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_eq ! (exp , act) ; } }
};
}
