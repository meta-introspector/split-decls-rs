// Generated macro for IS_GOPHER_DIRECTORY (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_DIRECTORY {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_DIRECTORY"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_DIRECTORY (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_DIRECTORY) != 0 { TRUE } else { FALSE } }
};
}
