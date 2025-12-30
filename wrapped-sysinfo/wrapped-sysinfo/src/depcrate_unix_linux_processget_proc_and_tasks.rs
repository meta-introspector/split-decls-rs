// Generated macro for get_proc_and_tasks (function)
macro_rules! Depcrate_unix_linux_processget_proc_and_tasks {
() => {
// Module: crate::unix::linux::process
// Provides: {"get_proc_and_tasks"}
// Dependencies: {}
fn get_proc_and_tasks (path : PathBuf , pid : Pid , refresh_kind : ProcessRefreshKind , processes_to_update : ProcessesToUpdate < '_ > ,) -> Vec < ProcAndTasks > { let mut parent_pid = None ; let (mut procs , mut tasks) = if refresh_kind . tasks () { let procs = get_proc_tasks (& path , pid) ; let tasks = procs . iter () . map (| ProcAndTasks { pid , .. } | * pid) . collect () ; (procs , Some (tasks)) } else { (Vec :: new () , None) } ; if processes_to_update != ProcessesToUpdate :: All { if let Some (tgid) = get_tgid (& path . join ("status")) && tgid != pid { parent_pid = Some (tgid) ; tasks = None ; } procs . clear () ; } procs . push (ProcAndTasks { pid , parent_pid , path , tasks , }) ; procs }
};
}
