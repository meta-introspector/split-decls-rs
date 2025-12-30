// Generated macro for refresh_tz (function)
macro_rules! Depcrate_sys_refresh_tzrefresh_tz {
() => {
// Module: crate::sys::refresh_tz
// Provides: {"refresh_tz"}
// Dependencies: {}
# [doc = " Attempt to update time zone information from the system."] # [doc = ""] # [doc = " Returns `None` if the call is not known to be sound."] # [inline] pub (crate) fn refresh_tz () -> Option < () > { imp :: refresh_tz () }
};
}
