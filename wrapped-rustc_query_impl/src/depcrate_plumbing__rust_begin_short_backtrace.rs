// Generated macro for __rust_begin_short_backtrace (function)
macro_rules! Depcrate_plumbing__rust_begin_short_backtrace {
() => {
// Module: crate::plumbing
// Provides: {"__rust_begin_short_backtrace"}
// Dependencies: {}
# [doc = " Don't show the backtrace for query system by default"] # [doc = " use `RUST_BACKTRACE=full` to show all the backtraces"] # [inline (never)] pub (crate) fn __rust_begin_short_backtrace < F , T > (f : F) -> T where F : FnOnce () -> T , { let result = f () ; std :: hint :: black_box (()) ; result }
};
}
