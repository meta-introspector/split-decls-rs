// Generated macro for get_proc_tasks (function)
macro_rules! Depcrate_unix_linux_processget_proc_tasks {
() => {
// Module: crate::unix::linux::process
// Provides: {"get_proc_tasks"}
// Dependencies: {}
fn get_proc_tasks (path : & Path , parent_pid : Pid) -> Vec < ProcAndTasks > { let task_path = path . join ("task") ; read_dir (task_path) . ok () . map (| task_entries | { task_entries . filter_map (filter_pid_entries) . filter (| (_ , pid) | * pid != parent_pid) . map (| (path , pid) | ProcAndTasks { pid , path , parent_pid : Some (parent_pid) , tasks : None , }) . collect () }) . unwrap_or_default () }
};
}
