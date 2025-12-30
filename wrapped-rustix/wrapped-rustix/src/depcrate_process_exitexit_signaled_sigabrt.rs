// Generated macro for EXIT_SIGNALED_SIGABRT (const)
macro_rules! Depcrate_process_exitEXIT_SIGNALED_SIGABRT {
() => {
// Module: crate::process::exit
// Provides: {"EXIT_SIGNALED_SIGABRT"}
// Dependencies: {}
# [doc = " The exit status used by a process terminated with a [`Signal::ABORT`]"] # [doc = " signal."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://tldp.org/LDP/abs/html/exitcodes.html"] # [doc = " [`Signal::ABORT`]: crate::process::Signal::ABORT"] # [cfg (not (any (target_os = "espidf" , target_os = "wasi")))] pub const EXIT_SIGNALED_SIGABRT : i32 = backend :: c :: EXIT_SIGNALED_SIGABRT ;
};
}
