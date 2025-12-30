// Generated macro for impl_519 (impl)
macro_rules! Depcrate_unix_apple_processimpl_519 {
() => {
// Module: crate::unix::apple::process
// Provides: {"impl_519"}
// Dependencies: {}
impl ProcessInner { pub (crate) fn open_files (& self) -> Option < usize > { let buffer_size_bytes = unsafe { libc :: proc_pidinfo (self . pid () . 0 , libc :: PROC_PIDLISTFDS , 0 , std :: ptr :: null_mut () , 0 ,) } ; if buffer_size_bytes < 0 { sysinfo_debug ! ("proc_pidinfo failed") ; None } else { Some (buffer_size_bytes as usize / std :: mem :: size_of :: < libc :: proc_fdinfo > ()) } } pub (crate) fn open_files_limit (& self) -> Option < usize > { crate :: System :: open_files_limit () } }
};
}
