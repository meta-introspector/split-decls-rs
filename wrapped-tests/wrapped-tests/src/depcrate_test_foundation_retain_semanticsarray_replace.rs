// Generated macro for array_replace (function)
macro_rules! Depcrate_test_foundation_retain_semanticsarray_replace {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"array_replace"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn array_replace () { let array = NSMutableArray :: new () ; let obj1 = RcTestObject :: new () ; let obj2 = RcTestObject :: new () ; array . addObject (& * obj1) ; let mut expected = ThreadTestData :: current () ; array . replaceObjectAtIndex_withObject (0 , & obj2) ; expected . retain += 1 ; expected . release += 1 ; expected . assert_current () ; }
};
}
