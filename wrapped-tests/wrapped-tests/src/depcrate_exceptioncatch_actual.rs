// Generated macro for catch_actual (function)
macro_rules! Depcrate_exceptioncatch_actual {
() => {
// Module: crate::exception
// Provides: {"catch_actual"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "catch-all" , ignore = "Panics inside `catch` when catch-all is enabled")] fn catch_actual () { let res = catch (| | { let arr : Retained < NSArray < NSObject > > = NSArray :: new () ; let _obj : * mut NSObject = unsafe { msg_send ! [& arr , objectAtIndex : 0usize] } ; }) ; let exc = res . unwrap_err () . unwrap () ; let name = "NSRangeException" ; let reason = if cfg ! (feature = "gnustep-1-7") { "Index 0 is out of range 0 (in 'objectAtIndex:')" } else { "*** -[__NSArray0 objectAtIndex:]: index 0 beyond bounds for empty " } ; assert ! (format ! ("{}" , exc) . starts_with (reason)) ; assert ! (format ! ("{:?}" , exc) . starts_with (& format ! ("exception <NSException: {:p}> '{}' reason: {}" , &* exc , name , reason))) ; let exc = NSException :: from_exception (exc) . unwrap () ; assert_eq ! (exc . name () , NSString :: from_str (name)) ; assert ! (exc . reason () . unwrap () . to_string () . starts_with (reason)) ; let user_info = exc . userInfo () ; if cfg ! (feature = "gnustep-1-7") { let user_info = user_info . unwrap () ; assert_eq ! (user_info . len () , 3) ; } else { assert ! (user_info . is_none ()) ; } }
};
}
