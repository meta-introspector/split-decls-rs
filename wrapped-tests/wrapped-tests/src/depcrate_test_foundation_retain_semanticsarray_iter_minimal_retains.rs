// Generated macro for array_iter_minimal_retains (function)
macro_rules! Depcrate_test_foundation_retain_semanticsarray_iter_minimal_retains {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"array_iter_minimal_retains"}
// Dependencies: {}
# [test] # [cfg_attr (target_vendor = "apple" , ignore = "this works differently on different framework versions")] fn array_iter_minimal_retains () { let objs = [RcTestObject :: new ()] ; let array = NSArray :: from_retained_slice (& objs) ; drop (objs) ; let mut expected = ThreadTestData :: current () ; let mut iter = array . iter () ; expected . assert_current () ; assert ! (iter . next () . is_some ()) ; expected . retain += 1 ; expected . release += 1 ; expected . assert_current () ; assert_eq ! (iter . count () , 0) ; expected . assert_current () ; let mut iter = unsafe { array . iter_unchecked () } ; expected . assert_current () ; assert ! (iter . next () . is_some ()) ; expected . assert_current () ; assert_eq ! (iter . count () , 0) ; expected . assert_current () ; let mut iter = array . into_iter () ; expected . assert_current () ; assert ! (iter . next () . is_some ()) ; expected . retain += 1 ; expected . release += 1 ; expected . assert_current () ; assert_eq ! (iter . count () , 0) ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; }
};
}
