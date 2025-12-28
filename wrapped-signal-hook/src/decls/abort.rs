macro_rules! abort {
    () => {
        # [doc = " A bare libc abort."] # [doc = ""] # [doc = " Unlike the [std::process::abort], this one is guaranteed to contain no additions or wrappers"] # [doc = " and therefore is async-signal-safe. You can use this to terminate the application from within a"] # [doc = " signal handler."] pub fn abort () -> ! { unsafe { libc :: abort () ; } }
    };
}

abort!();