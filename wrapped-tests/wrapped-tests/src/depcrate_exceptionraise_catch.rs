// Generated macro for raise_catch (function)
macro_rules! Depcrate_exceptionraise_catch {
() => {
// Module: crate::exception
// Provides: {"raise_catch"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "catch-all" , ignore = "Panics inside `catch` when catch-all is enabled")] fn raise_catch () { let name = NSString :: from_str ("abc") ; let reason = NSString :: from_str ("def") ; let exc = NSException :: new (& name , Some (& reason) , None) . unwrap () ; assert_eq ! (exc . retainCount () , 1) ; let exc = autoreleasepool (| pool | { let exc = unsafe { Retained :: autorelease (exc , pool) } ; let res = catch (| | { if exc . name () == name { exc . raise () ; } else { 42 } }) . unwrap_err () . unwrap () ; assert_eq ! (exc . retainCount () , 2) ; res }) ; let retain_count : usize = unsafe { msg_send ! [& exc , retainCount] } ; assert_eq ! (retain_count , 1) ; assert_eq ! (format ! ("{exc}") , "def") ; assert_eq ! (format ! ("{exc:?}") , format ! ("exception <NSException: {:p}> 'abc' reason: def" , &* exc)) ; }
};
}
