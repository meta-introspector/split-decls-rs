// Generated macro for ProcAndTasks (struct)
macro_rules! Depcrate_unix_linux_processProcAndTasks {
() => {
// Module: crate::unix::linux::process
// Provides: {"ProcAndTasks"}
// Dependencies: {}
struct ProcAndTasks { pid : Pid , parent_pid : Option < Pid > , path : PathBuf , tasks : Option < HashSet < Pid > > , }
};
}
