// Generated macro for HTTPAPI_GREATER_VERSION (function)
macro_rules! Depcrate_um_httpHTTPAPI_GREATER_VERSION {
() => {
// Module: crate::um::http
// Provides: {"HTTPAPI_GREATER_VERSION"}
// Dependencies: {}
# [inline] pub fn HTTPAPI_GREATER_VERSION (version : HTTPAPI_VERSION , major : USHORT , minor : USHORT) -> bool { version . HttpApiMajorVersion > major || (version . HttpApiMajorVersion == major && version . HttpApiMinorVersion > minor) }
};
}
