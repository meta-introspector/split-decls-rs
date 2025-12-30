// Generated macro for array_adding (function)
macro_rules! Depcrate_test_foundation_retain_semanticsarray_adding {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"array_adding"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn array_adding () { let array = NSMutableArray :: new () ; let obj1 = RcTestObject :: new () ; let obj2 = RcTestObject :: new () ; let mut expected = ThreadTestData :: current () ; array . addObject (& * obj1) ; expected . retain += 1 ; expected . assert_current () ; assert_eq ! (array . len () , 1) ; assert_eq ! (unsafe { array . objectAtIndex_unchecked (0) } , &* obj1) ; array . insert (0 , & obj2) ; expected . retain += 1 ; expected . assert_current () ; assert_eq ! (array . len () , 2) ; assert_eq ! (unsafe { array . objectAtIndex_unchecked (0) } , &* obj2) ; }
};
}
