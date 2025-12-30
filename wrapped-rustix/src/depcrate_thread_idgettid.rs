// Generated macro for gettid (function)
macro_rules! Depcrate_thread_idgettid {
() => {
// Module: crate::thread::id
// Provides: {"gettid"}
// Dependencies: {}
# [doc = " `gettid()`—Returns the thread ID."] # [doc = ""] # [doc = " This returns the OS thread ID, which is not necessarily the same as the"] # [doc = " Rust's `std::thread::Thread::id` or the pthread ID."] # [doc = ""] # [doc = " This function always does a system call. To avoid this overhead, ask the"] # [doc = " thread runtime for the ID instead, for example using [`libc::gettid`] or"] # [doc = " [`origin::thread::current_id`]."] # [doc = ""] # [doc = " [`libc::gettid`]: https://docs.rs/libc/*/libc/fn.gettid.html"] # [doc = " [`origin::thread::current_id`]: https://docs.rs/origin/*/origin/thread/fn.current_id.html"] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/gettid.2.html"] # [inline] # [must_use] pub fn gettid () -> Pid { backend :: thread :: syscalls :: gettid () }
};
}
