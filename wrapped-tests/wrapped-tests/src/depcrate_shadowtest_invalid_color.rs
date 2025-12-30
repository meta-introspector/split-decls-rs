// Generated macro for test_invalid_color (function)
macro_rules! Depcrate_shadowtest_invalid_color {
() => {
// Module: crate::shadow
// Provides: {"test_invalid_color"}
// Dependencies: {}
# [doc = " Test what happens when setting a non-color."] # [doc = ""] # [doc = " We conservatively don't mark `-[UIColor setShadowColor:]` as safe because"] # [doc = " of this, see also <https://github.com/madsmtm/objc2/issues/562>."] # [test] # [cfg_attr (feature = "catch-all" , ignore = "messes with our use of `exception::catch`")] fn test_invalid_color () { let shadow = NSShadow :: new () ; let color = unsafe { Retained :: cast_unchecked :: < NSColor > (NSString :: new ()) } ; shadow . setShadowColor (Some (& color)) ; let shadow = AssertUnwindSafe (shadow) ; let err = catch (| | shadow . set ()) . unwrap_err () . unwrap () . to_string () ; assert ! (err . contains ("CGColor") || err . contains ("colorUsingColorSpaceName:") , "{err:?} did not have expected message") ; assert ! (err . contains ("unrecognized selector sent to instance") , "{err:?} did not have expected message") ; }
};
}
