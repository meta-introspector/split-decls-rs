// Generated macro for HTTP_LESS_EQUAL_VERSION (function)
macro_rules! Depcrate_um_httpHTTP_LESS_EQUAL_VERSION {
() => {
// Module: crate::um::http
// Provides: {"HTTP_LESS_EQUAL_VERSION"}
// Dependencies: {}
# [inline] pub fn HTTP_LESS_EQUAL_VERSION (version : HTTP_VERSION , major : USHORT , minor : USHORT) -> bool { ! HTTP_GREATER_VERSION (version , major , minor) }
};
}
