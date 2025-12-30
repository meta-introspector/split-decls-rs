// Generated macro for value_does_not_retain (function)
macro_rules! Depcrate_test_foundation_retain_semanticsvalue_does_not_retain {
() => {
// Module: crate::test_foundation_retain_semantics
// Provides: {"value_does_not_retain"}
// Dependencies: {}
# [test] fn value_does_not_retain () { let obj = RcTestObject :: new () ; let expected = ThreadTestData :: current () ; let val = NSValue :: new :: < * const RcTestObject > (& * obj) ; expected . assert_current () ; assert ! (ptr :: eq (unsafe { val . get ::<* const RcTestObject > () } , &* obj)) ; expected . assert_current () ; let _clone = val . clone () ; expected . assert_current () ; let _copy = val . copy () ; expected . assert_current () ; drop (val) ; expected . assert_current () ; }
};
}
