// Generated macro for HTTPAPI_LESS_VERSION (function)
macro_rules! Depcrate_um_httpHTTPAPI_LESS_VERSION {
() => {
// Module: crate::um::http
// Provides: {"HTTPAPI_LESS_VERSION"}
// Dependencies: {}
# [inline] pub fn HTTPAPI_LESS_VERSION (version : HTTPAPI_VERSION , major : USHORT , minor : USHORT) -> bool { version . HttpApiMajorVersion < major || (version . HttpApiMajorVersion == major && version . HttpApiMinorVersion < minor) }
};
}
