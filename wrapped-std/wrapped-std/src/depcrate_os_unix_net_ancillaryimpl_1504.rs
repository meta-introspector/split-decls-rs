// Generated macro for impl_1504 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1504 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1504"}
// Dependencies: {}
# [doc (cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin")))] # [cfg (any (target_os = "android" , target_os = "linux" , target_os = "cygwin"))] impl SocketCred { # [doc = " Creates a Unix credential struct."] # [doc = ""] # [doc = " PID, UID and GID is set to 0."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [must_use] pub fn new () -> SocketCred { SocketCred (libc :: ucred { pid : 0 , uid : 0 , gid : 0 }) } # [doc = " Set the PID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_pid (& mut self , pid : libc :: pid_t) { self . 0 . pid = pid ; } # [doc = " Gets the current PID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_pid (& self) -> libc :: pid_t { self . 0 . pid } # [doc = " Set the UID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_uid (& mut self , uid : libc :: uid_t) { self . 0 . uid = uid ; } # [doc = " Gets the current UID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_uid (& self) -> libc :: uid_t { self . 0 . uid } # [doc = " Set the GID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_gid (& mut self , gid : libc :: gid_t) { self . 0 . gid = gid ; } # [doc = " Gets the current GID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_gid (& self) -> libc :: gid_t { self . 0 . gid } }
};
}
