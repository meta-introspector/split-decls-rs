macro_rules! __rust_begin_short_backtrace {
    () => {
        # [doc = " Don't show the backtrace for query system by default"] # [doc = " use `RUST_BACKTRACE=full` to show all the backtraces"] # [inline (never)] pub (crate) fn __rust_begin_short_backtrace < F , T > (f : F) -> T where F : FnOnce () -> T , { let result = f () ; std :: hint :: black_box (()) ; result }
    };
}

__rust_begin_short_backtrace!();