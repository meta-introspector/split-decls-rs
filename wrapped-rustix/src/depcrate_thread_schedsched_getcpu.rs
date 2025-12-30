// Generated macro for sched_getcpu (function)
macro_rules! Depcrate_thread_schedsched_getcpu {
() => {
// Module: crate::thread::sched
// Provides: {"sched_getcpu"}
// Dependencies: {}
# [doc = " `sched_getcpu()`—Get the CPU that the current thread is currently on."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = "  - [DragonFly BSD]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/sched_getcpu.3.html"] # [doc = " [DragonFly BSD]: https://man.dragonflybsd.org/?command=sched_getcpu&section=2"] # [cfg (any (linux_kernel , target_os = "dragonfly"))] # [inline] pub fn sched_getcpu () -> usize { backend :: thread :: syscalls :: sched_getcpu () }
};
}
