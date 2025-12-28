macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! configure_performance_counters {
    () => {
        deps!();
        # [doc = " Enable or disable all performance counters attached to the calling process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_TASK_PERF_EVENTS_ENABLE,…)`]"] # [doc = "  - [`prctl(PR_TASK_PERF_EVENTS_DISABLE,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_TASK_PERF_EVENTS_ENABLE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [doc = " [`prctl(PR_TASK_PERF_EVENTS_DISABLE,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_TASK_PERF_EVENTS_ENABLE")] # [doc (alias = "PR_TASK_PERF_EVENTS_DISABLE")] pub fn configure_performance_counters (enable : bool) -> io :: Result < () > { let option = if enable { PR_TASK_PERF_EVENTS_ENABLE } else { PR_TASK_PERF_EVENTS_DISABLE } ; unsafe { prctl_1arg (option) } . map (| _r | ()) }
    };
}

configure_performance_counters!();