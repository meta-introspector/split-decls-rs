// Generated macro for get_task_info (function)
macro_rules! Depcrate_unix_apple_macos_processget_task_info {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_task_info"}
// Dependencies: {}
unsafe fn get_task_info (pid : Pid) -> libc :: proc_taskinfo { unsafe { let mut task_info = mem :: zeroed :: < libc :: proc_taskinfo > () ; libc :: proc_pidinfo (pid . 0 , libc :: PROC_PIDTASKINFO , 0 , & mut task_info as * mut libc :: proc_taskinfo as * mut c_void , mem :: size_of :: < libc :: proc_taskinfo > () as _ ,) ; task_info } }
};
}
