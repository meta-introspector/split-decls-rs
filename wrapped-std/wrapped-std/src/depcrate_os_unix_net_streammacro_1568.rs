// Generated macro for macro_1568 (macro)
macro_rules! Depcrate_os_unix_net_streammacro_1568 {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"macro_1568"}
// Dependencies: {}
cfg_select ! { any (target_os = "linux" , target_os = "android" , target_os = "hurd" , target_os = "dragonfly" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd" , target_os = "solaris" , target_os = "illumos" , target_os = "haiku" , target_os = "nto" , target_os = "cygwin" ,) => { use libc :: MSG_NOSIGNAL ; } _ => { const MSG_NOSIGNAL : core :: ffi :: c_int = 0x0 ; } }
};
}
