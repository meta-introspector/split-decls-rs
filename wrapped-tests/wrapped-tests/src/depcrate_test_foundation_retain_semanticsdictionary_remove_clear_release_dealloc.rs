// Generated macro for dictionary_remove_clear_release_dealloc (function)
macro_rules! Depcrate_test_foundation_retain_semanticsdictionary_remove_clear_release_dealloc {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"dictionary_remove_clear_release_dealloc"}
// Dependencies: {}
# [test] fn dictionary_remove_clear_release_dealloc () { let dict = NSMutableDictionary :: new () ; for i in 0 .. 4 { dict . insert (& * NSNumber :: new_i32 (i) , & * RcTestObject :: new ()) ; } let mut expected = ThreadTestData :: current () ; dict . removeObjectForKey (& NSNumber :: new_i32 (1)) ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; assert_eq ! (dict . len () , 3) ; dict . removeObjectForKey (& NSNumber :: new_i32 (2)) ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; assert_eq ! (dict . len () , 2) ; dict . removeAllObjects () ; expected . release += 2 ; expected . drop += 2 ; expected . assert_current () ; assert_eq ! (dict . len () , 0) ; }
};
}
