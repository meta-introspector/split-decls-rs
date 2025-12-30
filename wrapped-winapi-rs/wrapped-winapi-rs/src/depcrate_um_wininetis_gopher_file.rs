// Generated macro for IS_GOPHER_FILE (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_FILE {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_FILE"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_FILE (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_FILE_MASK) != 0 { TRUE } else { FALSE } }
};
}
