// Generated macro for update_parent_pid (function)
macro_rules! Depcrate_unix_linux_processupdate_parent_pid {
() => {
// Module: crate::unix::linux::process
// Provides: {"update_parent_pid"}
// Dependencies: {}
fn update_parent_pid (p : & mut ProcessInner , parent_pid : Option < Pid > , str_parts : & [& str]) { p . parent = match parent_pid { Some (parent_pid) if parent_pid . 0 != 0 => Some (parent_pid) , _ => match Pid :: from_str (str_parts [ProcIndex :: ParentPid as usize]) { Ok (p) if p . 0 != 0 => Some (p) , _ => None , } , } ; }
};
}
