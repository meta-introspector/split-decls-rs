// Generated macro for is_proc_running (function)
macro_rules! Depcrate_windows_systemis_proc_running {
() => {
// Module: crate::windows::system
// Provides: {"is_proc_running"}
// Dependencies: {}
pub (crate) fn is_proc_running (handle : HANDLE) -> bool { let mut exit_code = 0 ; unsafe { GetExitCodeProcess (handle , & mut exit_code) } . is_ok () && exit_code == STILL_ACTIVE . 0 as u32 }
};
}
