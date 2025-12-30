// Generated macro for WT_SET_MAX_THREADPOOL_THREADS (function)
macro_rules! Depcrate_um_winntWT_SET_MAX_THREADPOOL_THREADS {
() => {
// Module: crate::um::winnt
// Provides: {"WT_SET_MAX_THREADPOOL_THREADS"}
// Dependencies: {}
# [inline] pub fn WT_SET_MAX_THREADPOOL_THREADS (Flags : ULONG , Limit : ULONG) -> ULONG { Flags | (Limit << 16) }
};
}
