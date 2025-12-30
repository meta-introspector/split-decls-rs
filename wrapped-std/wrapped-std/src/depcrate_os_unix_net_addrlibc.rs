// Generated macro for libc (module)
macro_rules! Depcrate_os_unix_net_addrlibc {
() => {
// Module: crate::os::unix::net::addr
// Provides: {"libc"}
// Dependencies: {}
# [cfg (not (unix))] # [allow (non_camel_case_types)] mod libc { pub use core :: ffi :: c_int ; pub type socklen_t = u32 ; pub struct sockaddr ; # [derive (Clone)] pub struct sockaddr_un { pub sun_path : [u8 ; 1] , } }
};
}
