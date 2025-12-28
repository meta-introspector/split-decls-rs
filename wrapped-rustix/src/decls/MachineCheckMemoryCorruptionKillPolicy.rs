macro_rules! MachineCheckMemoryCorruptionKillPolicy {
    () => {
        # [doc = " `PR_MCE_KILL_*` values for use with"] # [doc = " [`machine_check_memory_corruption_kill_policy`] and"] # [doc = " [`set_machine_check_memory_corruption_kill_policy`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum MachineCheckMemoryCorruptionKillPolicy { # [doc = " Late kill policy."] # [doc (alias = "PR_MCE_KILL_LATE")] Late = PR_MCE_KILL_LATE , # [doc = " Early kill policy."] # [doc (alias = "PR_MCE_KILL_EARLY")] Early = PR_MCE_KILL_EARLY , # [doc = " System-wide default policy."] # [doc (alias = "PR_MCE_KILL_DEFAULT")] Default = PR_MCE_KILL_DEFAULT , }
    };
}

MachineCheckMemoryCorruptionKillPolicy!()