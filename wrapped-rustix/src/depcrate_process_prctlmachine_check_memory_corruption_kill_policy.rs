// Generated macro for machine_check_memory_corruption_kill_policy (function)
macro_rules! Depcrate_process_prctlmachine_check_memory_corruption_kill_policy {
() => {
// Module: crate::process::prctl
// Provides: {"machine_check_memory_corruption_kill_policy"}
// Dependencies: {}
# [doc = " Get the current per-process machine check kill policy."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_MCE_KILL_GET,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_MCE_KILL_GET,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_MCE_KILL_GET")] pub fn machine_check_memory_corruption_kill_policy () -> io :: Result < MachineCheckMemoryCorruptionKillPolicy > { let r = unsafe { prctl_1arg (PR_MCE_KILL_GET) ? } as c_uint ; MachineCheckMemoryCorruptionKillPolicy :: try_from (r) }
};
}
