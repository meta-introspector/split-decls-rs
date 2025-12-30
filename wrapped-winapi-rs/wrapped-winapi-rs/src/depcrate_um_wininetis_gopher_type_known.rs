// Generated macro for IS_GOPHER_TYPE_KNOWN (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_TYPE_KNOWN {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_TYPE_KNOWN"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_TYPE_KNOWN (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_UNKNOWN) != 0 { FALSE } else { TRUE } }
};
}
