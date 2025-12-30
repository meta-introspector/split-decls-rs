// Generated macro for test (module)
macro_rules! Depcrate_filter_redactionstest {
() => {
// Module: crate::filter::redactions
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_validate_placeholder () { let cases = [("[HELLO" , false) , ("HELLO]" , false) , ("[HELLO]" , true) , ("[HELLO_WORLD]" , true) , ("[hello]" , false) , ("[HE  O]" , false) ,] ; for (placeholder , expected) in cases { let actual = validate_placeholder (placeholder) . is_ok () ; assert_eq ! (expected , actual , "placeholder={placeholder:?}") ; } } }
};
}
