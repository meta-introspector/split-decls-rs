// Generated macro for filter_pid_entries (function)
macro_rules! Depcrate_unix_linux_processfilter_pid_entries {
() => {
// Module: crate::unix::linux::process
// Provides: {"filter_pid_entries"}
// Dependencies: {}
fn filter_pid_entries (entry : Result < DirEntry , std :: io :: Error >) -> Option < (PathBuf , Pid) > { if let Ok (entry) = entry && let Ok (file_type) = entry . file_type () && file_type . is_dir () && let Some (name) = entry . file_name () . to_str () && let Ok (pid) = usize :: from_str (name) { Some ((entry . path () , Pid :: from (pid))) } else { None } }
};
}
