// Generated macro for array_remove (function)
macro_rules! Depcrate_test_foundation_retain_semanticsarray_remove {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"array_remove"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "gnustep-1-7" , ignore = "thread safety issues regarding initialization")] fn array_remove () { let array = NSMutableArray :: new () ; for _ in 0 .. 4 { array . addObject (& * RcTestObject :: new ()) ; } let mut expected = ThreadTestData :: current () ; array . removeObjectAtIndex (1) ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; assert_eq ! (array . len () , 3) ; array . removeLastObject () ; expected . release += 1 ; expected . drop += 1 ; expected . assert_current () ; assert_eq ! (array . len () , 2) ; array . removeAllObjects () ; expected . release += 2 ; expected . drop += 2 ; expected . assert_current () ; assert_eq ! (array . len () , 0) ; }
};
}
