// Generated macro for set_clear_release_dealloc (function)
macro_rules! Depcrate_test_foundation_retain_semanticsset_clear_release_dealloc {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"set_clear_release_dealloc"}
// Dependencies: {}
# [test] fn set_clear_release_dealloc () { let set = NSMutableSet :: new () ; for _ in 0 .. 4 { set . addObject (& * RcTestObject :: new ()) ; } let mut expected = ThreadTestData :: current () ; set . removeAllObjects () ; expected . release += 4 ; expected . drop += 4 ; expected . assert_current () ; assert_eq ! (set . len () , 0) ; }
};
}
