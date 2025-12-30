// Generated macro for retrieve_all_new_process_info (function)
macro_rules! Depcrate_unix_linux_processretrieve_all_new_process_info {
() => {
// Module: crate::unix::linux::process
// Provides: {"retrieve_all_new_process_info"}
// Dependencies: {}
fn retrieve_all_new_process_info (pid : Pid , parent_pid : Option < Pid > , parts : & Parts < '_ > , path : & Path , info : & SystemInfo , refresh_kind : ProcessRefreshKind , uptime : u64 ,) -> Process { let mut p = ProcessInner :: new (pid , path . to_owned ()) ; let mut proc_path = PathHandler :: new (path) ; let name = parts . short_exe ; let (start_time_raw , start_time_without_boot_time) = compute_start_time_without_boot_time (parts , info) ; p . start_time_raw = start_time_raw ; p . start_time_without_boot_time = start_time_without_boot_time ; p . start_time = p . start_time_without_boot_time . saturating_add (info . boot_time) ; p . name = OsStr :: from_bytes (name) . to_os_string () ; if c_ulong :: from_str (parts . str_parts [ProcIndex :: Flags as usize]) . map (| flags | flags & libc :: PF_KTHREAD as c_ulong != 0) . unwrap_or (false) { p . thread_kind = Some (ThreadKind :: Kernel) ; } else if parent_pid . is_some () { p . thread_kind = Some (ThreadKind :: Userland) ; } update_proc_info (& mut p , parent_pid , refresh_kind , & mut proc_path , & parts . str_parts , uptime , info ,) ; Process { inner : p } }
};
}
