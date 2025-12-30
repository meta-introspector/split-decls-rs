// Generated macro for add_missing_proc_info (function)
macro_rules! Depcrate_unix_freebsd_systemadd_missing_proc_info {
() => {
// Module: crate::unix::freebsd::system
// Provides: {"add_missing_proc_info"}
// Dependencies: {}
unsafe fn add_missing_proc_info (system_info : & mut SystemInfo , kproc : & libc :: kinfo_proc , proc_ : & mut Process , refresh_kind : ProcessRefreshKind ,) { { let kd = system_info . kd . as_ptr () ; let proc_inner = & mut proc_ . inner ; let cmd_needs_update = refresh_kind . cmd () . needs_update (| | proc_inner . cmd . is_empty ()) ; if proc_inner . name . is_empty () || cmd_needs_update { let cmd = unsafe { from_cstr_array (libc :: kvm_getargv (kd , kproc , 0) as _) } ; if ! cmd . is_empty () { let p = Path :: new (& cmd [0]) ; if let Some (name) = p . file_name () { name . clone_into (& mut proc_inner . name) ; } if cmd_needs_update { proc_inner . cmd = cmd ; } } } unsafe { get_exe (& mut proc_inner . exe , proc_inner . pid , refresh_kind) ; system_info . get_proc_missing_info (kproc , proc_inner , refresh_kind) ; } if proc_inner . name . is_empty () { proc_inner . name = c_buf_to_os_string (& kproc . ki_comm) ; } if refresh_kind . environ () . needs_update (| | proc_inner . environ . is_empty ()) { proc_inner . environ = unsafe { from_cstr_array (libc :: kvm_getenvv (kd , kproc , 0) as _) } ; } } }
};
}
