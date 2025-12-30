// Generated macro for HTTP_LESS_VERSION (function)
macro_rules! Depcrate_um_httpHTTP_LESS_VERSION {
() => {
// Module: crate::um::http
// Provides: {"HTTP_LESS_VERSION"}
// Dependencies: {}
# [inline] pub fn HTTP_LESS_VERSION (version : HTTP_VERSION , major : USHORT , minor : USHORT) -> bool { version . MajorVersion < major || (version . MajorVersion == major && version . MinorVersion < minor) }
};
}
