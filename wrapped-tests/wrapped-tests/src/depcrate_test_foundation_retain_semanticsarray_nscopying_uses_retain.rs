// Generated macro for array_nscopying_uses_retain (function)
macro_rules! Depcrate_test_foundation_retain_semanticsarray_nscopying_uses_retain {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"array_nscopying_uses_retain"}
// Dependencies: {}
# [test] fn array_nscopying_uses_retain () { let obj = RcTestObject :: new () ; let array = NSArray :: from_retained_slice (& [obj]) ; let mut expected = ThreadTestData :: current () ; let _copy = array . copy () ; expected . assert_current () ; let _copy = array . mutableCopy () ; expected . retain += 1 ; expected . assert_current () ; }
};
}
