// Generated macro for libc (module)
macro_rules! Depcrate_os_unix_net_ancillarylibc {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"libc"}
// Dependencies: {}
# [cfg (all (doc , not (target_os = "linux") , not (target_os = "android") , not (target_os = "netbsd") , not (target_os = "freebsd") , not (target_os = "cygwin") ,))] # [allow (non_camel_case_types)] mod libc { pub use core :: ffi :: c_int ; pub struct ucred ; pub struct cmsghdr ; pub struct sockcred2 ; pub type pid_t = i32 ; pub type gid_t = u32 ; pub type uid_t = u32 ; }
};
}
