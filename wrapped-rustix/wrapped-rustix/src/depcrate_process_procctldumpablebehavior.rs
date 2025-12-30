// Generated macro for DumpableBehavior (enum)
macro_rules! Depcrate_process_procctlDumpableBehavior {
() => {
// Module: crate::process::procctl
// Provides: {"DumpableBehavior"}
// Dependencies: {}
# [doc = " `PROC_TRACE_CTL_*`"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (i32)] pub enum DumpableBehavior { # [doc = " Not dumpable."] NotDumpable = PROC_TRACE_CTL_DISABLE , # [doc = " Dumpable."] Dumpable = PROC_TRACE_CTL_ENABLE , # [doc = " Not dumpable, and this behaviour is preserved across `execve` calls."] NotDumpableExecPreserved = PROC_TRACE_CTL_DISABLE_EXEC , }
};
}
