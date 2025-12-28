macro_rules! EXIT_SIGNALED_SIGABRT {
    () => {
        # [doc = " The exit status used by a process terminated with a [`Signal::ABORT`]"] # [doc = " signal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://tldp.org/LDP/abs/html/exitcodes.html"] # [doc = " [`Signal::ABORT`]: crate::process::Signal::ABORT"] # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] pub const EXIT_SIGNALED_SIGABRT : i32 = backend :: c :: EXIT_SIGNALED_SIGABRT ;
    };
}

EXIT_SIGNALED_SIGABRT!();