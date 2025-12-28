macro_rules! deps {
    () => {
        Pid!();
        CoreSchedulingScope!();
        Result!();
    };
}

macro_rules! create_core_scheduling_cookie {
    () => {
        deps!();
        # [doc = " Create unique core scheduling cookie."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_CREATE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SCHED_CORE,PR_SCHED_CORE_CREATE,…)`]: https://www.kernel.org/doc/html/v6.13/admin-guide/hw-vuln/core-scheduling.html"] # [inline] pub fn create_core_scheduling_cookie (pid : Pid , scope : CoreSchedulingScope) -> io :: Result < () > { unsafe { syscalls :: prctl (PR_SCHED_CORE , PR_SCHED_CORE_CREATE as * mut _ , pid . as_raw_nonzero () . get () as usize as * mut _ , scope as usize as * mut _ , ptr :: null_mut () ,) . map (| _r | ()) } }
    };
}

create_core_scheduling_cookie!()