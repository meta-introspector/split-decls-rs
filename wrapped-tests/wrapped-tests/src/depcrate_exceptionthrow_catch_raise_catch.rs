// Generated macro for throw_catch_raise_catch (function)
macro_rules! Depcrate_exceptionthrow_catch_raise_catch {
() => {
// Module: crate::exception
// Provides: {"throw_catch_raise_catch"}
// Dependencies: {}
# [test] # [cfg_attr (feature = "catch-all" , ignore = "Panics inside `catch` when catch-all is enabled")] fn throw_catch_raise_catch () { let name = NSString :: from_str ("abc") ; let reason = NSString :: from_str ("def") ; let exc = NSException :: new (& name , Some (& reason) , None) . unwrap () ; assert_eq ! (exc . retainCount () , 1) ; let exc = autoreleasepool (| _ | { let exc = NSException :: into_exception (exc) ; let res = catch (| | throw (exc)) ; let exc = res . unwrap_err () . unwrap () ; let exc = NSException :: from_exception (exc) . unwrap () ; if cfg ! (all (target_os = "macos" , target_arch = "x86")) { assert_eq ! (exc . retainCount () , 2) ; } else { assert_eq ! (exc . retainCount () , 1) ; } exc }) ; if cfg ! (all (target_os = "macos" , target_arch = "x86")) { assert_eq ! (exc . retainCount () , 2) ; } else { assert_eq ! (exc . retainCount () , 1) ; } let exc = autoreleasepool (| _ | { let res = catch (| | { autoreleasepool (| pool | { let exc = unsafe { Retained :: autorelease (exc , pool) } ; exc . raise () }) }) ; let exc = NSException :: from_exception (res . unwrap_err () . unwrap ()) . unwrap () ; if cfg ! (all (target_os = "macos" , target_arch = "x86")) { assert_eq ! (exc . retainCount () , 3) ; } else { assert_eq ! (exc . retainCount () , 1) ; } exc }) ; assert_eq ! (exc . retainCount () , 1) ; assert_eq ! (exc . name () , name) ; assert_eq ! (exc . reason () . unwrap () , reason) ; assert ! (exc . userInfo () . is_none ()) ; }
};
}
