macro_rules! deps {
    () => {
        CoreSchedulingScope!();
        Pid!();
        Result!();
    };
}

macro_rules! pull_core_scheduling_cookie {
    () => {
        deps!();
        # [doc = " Pull core scheduling cookie from a process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_SHARE_FROM,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_SHARE_FROM,…)`]: https://www.kernel.org/doc/html/v6.13/admin-guide/hw-vuln/core-scheduling.html"] # [inline] pub fn pull_core_scheduling_cookie (pid : Pid , scope : CoreSchedulingScope) -> io :: Result < () > { unsafe { syscalls :: prctl (PR_SCHED_CORE , PR_SCHED_CORE_SHARE_FROM as * mut _ , pid . as_raw_nonzero () . get () as usize as * mut _ , scope as usize as * mut _ , ptr :: null_mut () ,) . map (| _r | ()) } }
    };
}

pull_core_scheduling_cookie!();