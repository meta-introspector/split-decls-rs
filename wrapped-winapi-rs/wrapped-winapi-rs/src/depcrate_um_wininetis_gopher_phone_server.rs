// Generated macro for IS_GOPHER_PHONE_SERVER (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_PHONE_SERVER {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_PHONE_SERVER"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_PHONE_SERVER (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_CSO) != 0 { TRUE } else { FALSE } }
};
}
