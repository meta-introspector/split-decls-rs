// Generated macro for IS_GOPHER_ERROR (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_ERROR {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_ERROR"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_ERROR (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_ERROR) != 0 { TRUE } else { FALSE } }
};
}
