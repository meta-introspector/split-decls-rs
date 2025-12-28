macro_rules! deps {
    () => {
        MachineCheckMemoryCorruptionKillPolicy!();
        Result!();
    };
}

macro_rules! set_machine_check_memory_corruption_kill_policy {
    () => {
        deps!();
        # [doc = " Set the machine check memory corruption kill policy for the calling thread."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_MCE_KILL,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_MCE_KILL,…)`]: https://man7.org/linux/man-pages/man2/prctl.2.html"] # [inline] # [doc (alias = "PR_MCE_KILL")] pub fn set_machine_check_memory_corruption_kill_policy (policy : Option < MachineCheckMemoryCorruptionKillPolicy > ,) -> io :: Result < () > { let (sub_operation , policy) = if let Some (policy) = policy { (PR_MCE_KILL_SET , policy as usize as * mut _) } else { (PR_MCE_KILL_CLEAR , null_mut ()) } ; unsafe { prctl_3args (PR_MCE_KILL , sub_operation as * mut _ , policy) } . map (| _r | ()) }
    };
}

set_machine_check_memory_corruption_kill_policy!()