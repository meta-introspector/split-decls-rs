// Generated macro for raise_catch_all2 (function)
macro_rules! Depcrate_exceptionraise_catch_all2 {
() => {
// Module: crate::exception
// Provides: {"raise_catch_all2"}
// Dependencies: {}
# [test] # [cfg (feature = "catch-all")] # [should_panic = "> 'abc' reason: def"] fn raise_catch_all2 () { let name = NSString :: from_str ("abc") ; let reason = NSString :: from_str ("def") ; let exc = NSException :: new (& name , Some (& reason) , None) . unwrap () ; exc . raise () ; }
};
}
