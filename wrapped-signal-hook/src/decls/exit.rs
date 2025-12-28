macro_rules! exit {
    () => {
        # [doc = " A bare libc exit."] # [doc = ""] # [doc = " Unlike the [std::process::exit], this one is guaranteed to contain no additions or wrappers and"] # [doc = " therefore is async-signal-safe. You can use this to terminate the application from within a"] # [doc = " signal handler."] # [doc = ""] # [doc = " Also, see [`register_conditional_shutdown`][crate::flag::register_conditional_shutdown]."] pub fn exit (status : c_int) -> ! { unsafe { libc :: _exit (status) ; } }
    };
}

exit!();