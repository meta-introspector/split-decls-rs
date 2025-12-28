macro_rules! deps {
    () => {
        Result!();
        Pid!();
        CoreSchedulingScope!();
    };
}

macro_rules! push_core_scheduling_cookie {
    () => {
        deps!();
        # [doc = " Push core scheduling cookie to a process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_SHARE_TO,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_SHARE_TO,…)`]: https://www.kernel.org/doc/html/v6.13/admin-guide/hw-vuln/core-scheduling.html"] # [inline] pub fn push_core_scheduling_cookie (pid : Pid , scope : CoreSchedulingScope) -> io :: Result < () > { unsafe { syscalls :: prctl (PR_SCHED_CORE , PR_SCHED_CORE_SHARE_TO as * mut _ , pid . as_raw_nonzero () . get () as usize as * mut _ , scope as usize as * mut _ , ptr :: null_mut () ,) . map (| _r | ()) } }
    };
}

push_core_scheduling_cookie!();