// Generated macro for IS_GOPHER_TN3270_SESSION (function)
macro_rules! Depcrate_um_wininetIS_GOPHER_TN3270_SESSION {
() => {
// Module: crate::um::wininet
// Provides: {"IS_GOPHER_TN3270_SESSION"}
// Dependencies: {}
# [inline] pub fn IS_GOPHER_TN3270_SESSION (type_ : DWORD) -> BOOL { if (type_ & GOPHER_TYPE_TN3270) != 0 { TRUE } else { FALSE } }
};
}
