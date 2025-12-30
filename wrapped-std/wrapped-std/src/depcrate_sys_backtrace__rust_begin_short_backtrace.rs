// Generated macro for __rust_begin_short_backtrace (function)
macro_rules! Depcrate_sys_backtrace__rust_begin_short_backtrace {
() => {
// Module: crate::sys::backtrace
// Provides: {"__rust_begin_short_backtrace"}
// Dependencies: {}
# [doc = " Fixed frame used to clean the backtrace with `RUST_BACKTRACE=1`. Note that"] # [doc = " this is only inline(never) when backtraces in std are enabled, otherwise"] # [doc = " it's fine to optimize away."] # [cfg_attr (feature = "backtrace" , inline (never))] pub fn __rust_begin_short_backtrace < F , T > (f : F) -> T where F : FnOnce () -> T , { let result = f () ; crate :: hint :: black_box (()) ; result }
};
}
