// Generated macro for IS_TARGET_UNWIND (function)
macro_rules! Depcrate_um_winntIS_TARGET_UNWIND {
() => {
// Module: crate::um::winnt
// Provides: {"IS_TARGET_UNWIND"}
// Dependencies: {}
# [inline] pub fn IS_TARGET_UNWIND (Flag : DWORD) -> bool { (Flag & EXCEPTION_TARGET_UNWIND) != 0 }
};
}
