// Generated macro for set_insert_retain_release (function)
macro_rules! Depcrate_test_foundation_retain_semanticsset_insert_retain_release {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"set_insert_retain_release"}
// Dependencies: {}
# [test] fn set_insert_retain_release () { let set = < NSMutableSet < RcTestObject > > :: new () ; let obj1 = RcTestObject :: new () ; let obj2 = RcTestObject :: new () ; let mut expected = ThreadTestData :: current () ; set . addObject (& obj1) ; expected . retain += 1 ; expected . assert_current () ; assert_eq ! (set . len () , 1) ; assert_eq ! (unsafe { set . anyObject_unchecked () . unwrap () } , &* obj1) ; set . addObject (& obj2) ; expected . retain += 1 ; expected . assert_current () ; assert_eq ! (set . len () , 2) ; set . addObject (& obj2) ; expected . retain += 0 ; expected . assert_current () ; assert_eq ! (set . len () , 2) ; }
};
}
