// Generated macro for core_scheduling_cookie (function)
macro_rules! Depcrate_thread_prctlcore_scheduling_cookie {
() => {
// Module: crate::thread::prctl
// Provides: {"core_scheduling_cookie"}
// Dependencies: {}
# [doc = " Get core scheduling cookie of a process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_GET,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_GET,…)`]: https://www.kernel.org/doc/html/v6.13/admin-guide/hw-vuln/core-scheduling.html"] # [inline] pub fn core_scheduling_cookie (pid : Pid , scope : CoreSchedulingScope) -> io :: Result < u64 > { let mut value : MaybeUninit < u64 > = MaybeUninit :: uninit () ; unsafe { syscalls :: prctl (PR_SCHED_CORE , PR_SCHED_CORE_GET as * mut _ , pid . as_raw_nonzero () . get () as usize as * mut _ , scope as usize as * mut _ , value . as_mut_ptr () . cast () ,) ? ; Ok (value . assume_init ()) } }
};
}
