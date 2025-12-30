// Generated macro for downcast_basics (function)
macro_rules! Depcrate_test_objectdowncast_basics {
() => {
// Module: crate::test_object
// Provides: {"downcast_basics"}
// Dependencies: {}
# [test] fn downcast_basics () { let obj = NSString :: new () ; assert ! (obj . downcast_ref ::< NSString > () . is_some ()) ; let obj = obj . into_super () ; assert ! (obj . downcast_ref ::< NSNumber > () . is_none ()) ; assert ! (obj . downcast_ref ::< NSString > () . is_some ()) ; let obj = NSMutableString :: new () ; assert ! (obj . downcast_ref ::< NSMutableString > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSString > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSObject > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSException > () . is_none ()) ; let obj = obj . into_super () . into_super () ; assert ! (obj . downcast_ref ::< NSMutableString > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSString > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSObject > () . is_some ()) ; assert ! (obj . downcast_ref ::< NSException > () . is_none ()) ; let obj : Retained < NSArray < NSString > > = NSArray :: new () ; assert ! (obj . downcast_ref ::< NSString > () . is_none ()) ; assert ! (obj . downcast_ref ::< NSArray < AnyObject >> () . is_some ()) ; }
};
}
