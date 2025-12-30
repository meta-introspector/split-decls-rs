// Generated macro for IS_UNWINDING (function)
macro_rules! Depcrate_um_winntIS_UNWINDING {
() => {
// Module: crate::um::winnt
// Provides: {"IS_UNWINDING"}
// Dependencies: {}
# [inline] pub fn IS_UNWINDING (Flag : DWORD) -> bool { (Flag & EXCEPTION_UNWIND) != 0 }
};
}
