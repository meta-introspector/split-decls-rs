// Generated macro for ProcSelector (type)
macro_rules! Depcrate_process_procctlProcSelector {
() => {
// Module: crate::process::procctl
// Provides: {"ProcSelector"}
// Dependencies: {}
# [doc = " A process selector for use with the `procctl` interface."] # [doc = ""] # [doc = " `None` represents the current process. `Some((IdType::Pid, pid))`"] # [doc = " represents the process with pid `pid`. `Some((IdType::Pgid, pgid))`"] # [doc = " represents the control processes belonging to the process group with id"] # [doc = " `pgid`."] pub type ProcSelector = Option < (IdType , Pid) > ;
};
}
