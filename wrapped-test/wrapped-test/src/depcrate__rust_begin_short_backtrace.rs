// Generated macro for __rust_begin_short_backtrace (function)
macro_rules! Depcrate__rust_begin_short_backtrace {
() => {
// Module: crate
// Provides: {"__rust_begin_short_backtrace"}
// Dependencies: {}
# [doc = " Fixed frame used to clean the backtrace with `RUST_BACKTRACE=1`."] # [inline (never)] fn __rust_begin_short_backtrace < T , F : FnOnce () -> T > (f : F) -> T { let result = f () ; black_box (result) }
};
}
