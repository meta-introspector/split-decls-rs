macro_rules! deps {
    () => {
        Result!();
        MachineCheckMemoryCorruptionKillPolicy!();
    };
}

macro_rules! machine_check_memory_corruption_kill_policy {
    () => {
        deps!();
        # [doc = " Get the current per-process machine check kill policy."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_MCE_KILL_GET,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_MCE_KILL_GET,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_MCE_KILL_GET")] pub fn machine_check_memory_corruption_kill_policy () -> io :: Result < MachineCheckMemoryCorruptionKillPolicy > { let r = unsafe { prctl_1arg (PR_MCE_KILL_GET) ? } as c_uint ; MachineCheckMemoryCorruptionKillPolicy :: try_from (r) }
    };
}

machine_check_memory_corruption_kill_policy!();