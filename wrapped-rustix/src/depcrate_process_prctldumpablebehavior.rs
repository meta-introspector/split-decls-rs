// Generated macro for DumpableBehavior (enum)
macro_rules! Depcrate_process_prctlDumpableBehavior {
() => {
// Module: crate::process::prctl
// Provides: {"DumpableBehavior"}
// Dependencies: {}
# [doc = " `SUID_DUMP_*` values for use with [`dumpable_behavior`] and"] # [doc = " [`set_dumpable_behavior`]."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (i32)] pub enum DumpableBehavior { # [doc = " Not dumpable."] # [doc (alias = "SUID_DUMP_DISABLE")] NotDumpable = SUID_DUMP_DISABLE , # [doc = " Dumpable."] # [doc (alias = "SUID_DUMP_USER")] Dumpable = SUID_DUMP_USER , # [doc = " Dumpable but only readable by root."] # [doc (alias = "SUID_DUMP_ROOT")] DumpableReadableOnlyByRoot = SUID_DUMP_ROOT , }
};
}
