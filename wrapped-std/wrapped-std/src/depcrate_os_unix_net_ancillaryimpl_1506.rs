// Generated macro for impl_1506 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1506 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1506"}
// Dependencies: {}
# [cfg (target_os = "netbsd")] impl SocketCred { # [doc = " Creates a Unix credential struct."] # [doc = ""] # [doc = " PID, UID and GID is set to 0."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn new () -> SocketCred { SocketCred (libc :: sockcred { sc_pid : 0 , sc_uid : 0 , sc_euid : 0 , sc_gid : 0 , sc_egid : 0 , sc_ngroups : 0 , sc_groups : [0u32 ; 1] , }) } # [doc = " Set the PID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_pid (& mut self , pid : libc :: pid_t) { self . 0 . sc_pid = pid ; } # [doc = " Gets the current PID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_pid (& self) -> libc :: pid_t { self . 0 . sc_pid } # [doc = " Set the UID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_uid (& mut self , uid : libc :: uid_t) { self . 0 . sc_uid = uid ; } # [doc = " Gets the current UID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_uid (& self) -> libc :: uid_t { self . 0 . sc_uid } # [doc = " Set the GID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_gid (& mut self , gid : libc :: gid_t) { self . 0 . sc_gid = gid ; } # [doc = " Gets the current GID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_gid (& self) -> libc :: gid_t { self . 0 . sc_gid } }
};
}
