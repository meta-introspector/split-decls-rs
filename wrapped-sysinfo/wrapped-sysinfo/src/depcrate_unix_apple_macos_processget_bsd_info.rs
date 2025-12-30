// Generated macro for get_bsd_info (function)
macro_rules! Depcrate_unix_apple_macos_processget_bsd_info {
() => {
// Module: crate::unix::apple::macos::process
// Provides: {"get_bsd_info"}
// Dependencies: {}
unsafe fn get_bsd_info (pid : Pid) -> Option < libc :: proc_bsdinfo > { unsafe { let mut info = mem :: zeroed :: < libc :: proc_bsdinfo > () ; if libc :: proc_pidinfo (pid . 0 , libc :: PROC_PIDTBSDINFO , 0 , & mut info as * mut _ as * mut _ , mem :: size_of :: < libc :: proc_bsdinfo > () as _ ,) != mem :: size_of :: < libc :: proc_bsdinfo > () as c_int { None } else { Some (info) } } }
};
}
