// Generated macro for set_nscopying_uses_retain (function)
macro_rules! Depcrate_test_foundation_retain_semanticsset_nscopying_uses_retain {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"set_nscopying_uses_retain"}
// Dependencies: {}
# [test] fn set_nscopying_uses_retain () { let obj = RcTestObject :: new () ; let set = NSSet :: from_retained_slice (& [obj]) ; let mut expected = ThreadTestData :: current () ; let _copy = set . copy () ; expected . assert_current () ; let _copy = set . mutableCopy () ; expected . retain += 1 ; expected . assert_current () ; }
};
}
