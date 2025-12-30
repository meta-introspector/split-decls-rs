// Generated macro for refresh_tz (function)
macro_rules! Depcrate_utilrefresh_tz {
() => {
// Module: crate::util
// Provides: {"refresh_tz"}
// Dependencies: {}
# [doc = " Attempt to update time zone information from the system."] # [doc = ""] # [doc = " Returns `None` if the call is not known to be sound."] # [cfg (feature = "local-offset")] # [inline] pub fn refresh_tz () -> Option < () > { crate :: sys :: refresh_tz () }
};
}
