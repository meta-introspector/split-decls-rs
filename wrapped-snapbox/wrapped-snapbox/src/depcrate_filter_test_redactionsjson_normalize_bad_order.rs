// Generated macro for json_normalize_bad_order (function)
macro_rules! Depcrate_filter_test_redactionsjson_normalize_bad_order {
() => {
// Module: crate::filter::test_redactions
// Provides: {"json_normalize_bad_order"}
// Dependencies: {}
# [test] # [cfg (feature = "json")] fn json_normalize_bad_order () { let exp = json ! ({ "people" : ["John" , "Jane"] }) ; let expected = Data :: json (exp) ; let actual = json ! ({ "people" : ["Jane" , "John"] }) ; let actual = NormalizeToExpected :: new () . redact () . normalize (Data :: json (actual) , & expected) ; if let (DataInner :: Json (exp) , DataInner :: Json (act)) = (expected . inner , actual . inner) { assert_ne ! (exp , act) ; } }
};
}
