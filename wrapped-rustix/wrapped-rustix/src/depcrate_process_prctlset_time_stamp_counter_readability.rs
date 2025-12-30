// Generated macro for set_time_stamp_counter_readability (function)
macro_rules! Depcrate_process_prctlset_time_stamp_counter_readability {
() => {
// Module: crate::process::prctl
// Provides: {"set_time_stamp_counter_readability"}
// Dependencies: {}
# [doc = " Set the state of the flag determining if the timestamp counter can be read"] # [doc = " by the process."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_SET_TSC,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_SET_TSC,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_SET_TSC")] pub fn set_time_stamp_counter_readability (readability : TimeStampCounterReadability ,) -> io :: Result < () > { unsafe { prctl_2args (PR_SET_TSC , readability as usize as * mut _) } . map (| _r | ()) }
};
}
