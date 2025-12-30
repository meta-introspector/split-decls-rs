// Generated macro for IS_GOPHER_TELNET_SESSION (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_TELNET_SESSION {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_TELNET_SESSION"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_TELNET_SESSION (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_TELNET) != 0 { TRUE } else { FALSE } }
};
}
