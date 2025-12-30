// Generated macro for HTTP_GREATER_EQUAL_VERSION (function)
macro_rules! Depcrate_um_httpHTTP_GREATER_EQUAL_VERSION {
() => {
// Module: crate::um::http
// Provides: {"HTTP_GREATER_EQUAL_VERSION"}
// Dependencies: {}
# [inline] pub fn HTTP_GREATER_EQUAL_VERSION (version : HTTP_VERSION , major : USHORT , minor : USHORT) -> bool { ! HTTP_LESS_VERSION (version , major , minor) }
};
}
