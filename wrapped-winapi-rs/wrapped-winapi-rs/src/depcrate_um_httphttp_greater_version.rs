// Generated macro for HTTP_GREATER_VERSION (function)
macro_rules! Depcrate_um_httpHTTP_GREATER_VERSION {
() => {
// Module: crate::um::http
// Provides: {"HTTP_GREATER_VERSION"}
// Dependencies: {}
# [inline] pub fn HTTP_GREATER_VERSION (version : HTTP_VERSION , major : USHORT , minor : USHORT) -> bool { version . MajorVersion > major || (version . MajorVersion == major && version . MinorVersion > minor) }
};
}
