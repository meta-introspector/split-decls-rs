// Generated macro for set_retains_stored (function)
macro_rules! Depcrate_test_foundation_retain_semanticsset_retains_stored {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"set_retains_stored"}
// Dependencies: {}
# [test] fn set_retains_stored () { let obj = RcTestObject :: new () ; let mut expected = ThreadTestData :: current () ; let input = [obj . clone () , obj . clone ()] ; expected . retain += 2 ; expected . assert_current () ; let set = NSSet :: from_retained_slice (& input) ; expected . retain += 1 ; expected . assert_current () ; drop (input) ; expected . release += 2 ; expected . assert_current () ; let _obj = unsafe { set . anyObject_unchecked () . unwrap () } ; expected . assert_current () ; drop (set) ; expected . release += 1 ; expected . assert_current () ; let set = NSSet :: from_slice (& [& * obj]) ; expected . retain += 1 ; expected . assert_current () ; drop (set) ; expected . release += 1 ; expected . assert_current () ; drop (obj) ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; }
};
}
