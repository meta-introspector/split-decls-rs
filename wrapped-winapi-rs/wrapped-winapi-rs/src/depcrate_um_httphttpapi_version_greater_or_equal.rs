// Generated macro for HTTPAPI_VERSION_GREATER_OR_EQUAL (function)
macro_rules! Depcrate_um_httpHTTPAPI_VERSION_GREATER_OR_EQUAL {
() => {
// Module: crate::um::http
// Provides: {"HTTPAPI_VERSION_GREATER_OR_EQUAL"}
// Dependencies: {}
# [inline] pub fn HTTPAPI_VERSION_GREATER_OR_EQUAL (version : HTTPAPI_VERSION , major : USHORT , minor : USHORT ,) -> bool { ! HTTPAPI_LESS_VERSION (version , major , minor) }
};
}
