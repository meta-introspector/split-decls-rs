// Generated macro for raise_catch_all1 (function)
macro_rules! Depcrate_exceptionraise_catch_all1 {
() => {
// Module: crate::exception
// Provides: {"raise_catch_all1"}
// Dependencies: {}
# [test] # [cfg (feature = "catch-all")] # [should_panic = "uncaught exception <NSException: 0x"] fn raise_catch_all1 () { let name = NSString :: from_str ("abc") ; let reason = NSString :: from_str ("def") ; let exc = NSException :: new (& name , Some (& reason) , None) . unwrap () ; exc . raise () ; }
};
}
