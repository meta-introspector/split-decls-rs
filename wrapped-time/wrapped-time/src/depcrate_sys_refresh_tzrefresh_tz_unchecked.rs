// Generated macro for refresh_tz_unchecked (function)
macro_rules! Depcrate_sys_refresh_tzrefresh_tz_unchecked {
() => {
// Module: crate::sys::refresh_tz
// Provides: {"refresh_tz_unchecked"}
// Dependencies: {}
# [doc = " Update time zone information from the system."] # [doc = ""] # [doc = " For safety documentation, see [`time::util::refresh_tz`]."] # [inline] pub (crate) unsafe fn refresh_tz_unchecked () { unsafe { imp :: refresh_tz_unchecked () } }
};
}
