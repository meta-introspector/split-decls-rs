// Generated macro for array_retains_stored (function)
macro_rules! Depcrate_test_foundation_retain_semanticsarray_retains_stored {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"array_retains_stored"}
// Dependencies: {}
# [test] fn array_retains_stored () { let obj = RcTestObject :: new () ; let mut expected = ThreadTestData :: current () ; let input = [obj . clone () , obj . clone ()] ; expected . retain += 2 ; expected . assert_current () ; let array = NSArray :: from_retained_slice (& input) ; expected . retain += 2 ; expected . assert_current () ; drop (input) ; expected . release += 2 ; expected . assert_current () ; let _obj = unsafe { array . firstObject_unchecked () } . unwrap () ; expected . assert_current () ; drop (array) ; expected . release += 2 ; expected . assert_current () ; let array = NSArray :: from_slice (& [& * obj , & * obj]) ; expected . retain += 2 ; expected . assert_current () ; let _obj = unsafe { array . objectAtIndex_unchecked (0) } ; let _obj = unsafe { array . objectAtIndex_unchecked (1) } ; assert_eq ! (array . len () , 2) ; expected . assert_current () ; drop (array) ; expected . release += 2 ; expected . assert_current () ; drop (obj) ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; }
};
}
