// Generated macro for assert_throws (function)
macro_rules! Depcrate_bound_checksassert_throws {
() => {
// Module: crate::bound_checks
// Provides: {"assert_throws"}
// Dependencies: {}
# [doc = " Check that the given error message was thrown by the closure."] # [track_caller] fn assert_throws < R : std :: fmt :: Debug > (message : & str , f : impl FnOnce () -> R) { let f = AssertUnwindSafe (f) ; if cfg ! (feature = "catch-all") { let err = std :: panic :: catch_unwind (f) . unwrap_err () ; if let Some (s) = err . downcast_ref :: < & str > () { assert ! (s . contains (message) , "{s:?} did not contain {message:?}") ; } else if let Some (s) = err . downcast_ref :: < std :: string :: String > () { assert ! (s . contains (message) , "{s:?} did not contain {message:?}") ; } else { panic ! ("Caught panic with unknown type") ; } } else { let exc = objc2 :: exception :: catch (f) . unwrap_err () . unwrap () ; let exc = exc . downcast_ref :: < NSException > () . unwrap () ; let reason = exc . reason () . unwrap () . to_string () ; assert ! (reason . contains (message) , "{}: {reason:?} did not contain {message:?}" , exc . name () ,) ; } }
};
}
