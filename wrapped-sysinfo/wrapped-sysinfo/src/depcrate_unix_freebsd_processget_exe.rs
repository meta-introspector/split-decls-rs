// Generated macro for get_exe (function)
macro_rules! Depcrate_unix_freebsd_processget_exe {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"get_exe"}
// Dependencies: {}
pub (crate) unsafe fn get_exe (exe : & mut Option < PathBuf > , pid : crate :: Pid , refresh_kind : ProcessRefreshKind ,) { if refresh_kind . exe () . needs_update (| | exe . is_none ()) { let mut buffer = [0 ; libc :: PATH_MAX as usize + 1] ; unsafe { * exe = get_sys_value_str (& [libc :: CTL_KERN , libc :: KERN_PROC , libc :: KERN_PROC_PATHNAME , pid . 0 ,] , & mut buffer ,) . map (PathBuf :: from) ; } } }
};
}
