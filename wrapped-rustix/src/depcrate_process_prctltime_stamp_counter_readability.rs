// Generated macro for time_stamp_counter_readability (function)
macro_rules! Depcrate_process_prctltime_stamp_counter_readability {
() => {
// Module: crate::process::prctl
// Provides: {"time_stamp_counter_readability"}
// Dependencies: {}
# [doc = " Get the state of the flag determining if the timestamp counter can be read."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_GET_TSC,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_GET_TSC,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_GET_TSC")] pub fn time_stamp_counter_readability () -> io :: Result < TimeStampCounterReadability > { unsafe { prctl_get_at_arg2 :: < c_uint , _ > (PR_GET_TSC) } }
};
}
