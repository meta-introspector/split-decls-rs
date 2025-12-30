// Generated macro for test_downcast_class (function)
macro_rules! Depcrate_test_objecttest_downcast_class {
() => {
// Module: crate::test_object
// Provides: {"test_downcast_class"}
// Dependencies: {}
# [test] fn test_downcast_class () { let cls = NSString :: class () ; let obj = unsafe { & * (cls as * const AnyClass) . cast :: < AnyObject > () } ; assert ! (obj . downcast_ref ::< NSObject > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSString > () . is_none ()) ; }
};
}
