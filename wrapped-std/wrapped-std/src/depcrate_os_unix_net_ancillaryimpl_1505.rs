// Generated macro for impl_1505 (impl)
macro_rules! Depcrate_os_unix_net_ancillaryimpl_1505 {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"impl_1505"}
// Dependencies: {}
# [cfg (target_os = "freebsd")] impl SocketCred { # [doc = " Creates a Unix credential struct."] # [doc = ""] # [doc = " PID, UID and GID is set to 0."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [must_use] pub fn new () -> SocketCred { SocketCred (libc :: sockcred2 { sc_version : 0 , sc_pid : 0 , sc_uid : 0 , sc_euid : 0 , sc_gid : 0 , sc_egid : 0 , sc_ngroups : 0 , sc_groups : [0 ; 1] , }) } # [doc = " Set the PID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_pid (& mut self , pid : libc :: pid_t) { self . 0 . sc_pid = pid ; } # [doc = " Gets the current PID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_pid (& self) -> libc :: pid_t { self . 0 . sc_pid } # [doc = " Set the UID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_uid (& mut self , uid : libc :: uid_t) { self . 0 . sc_euid = uid ; } # [doc = " Gets the current UID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_uid (& self) -> libc :: uid_t { self . 0 . sc_euid } # [doc = " Set the GID."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn set_gid (& mut self , gid : libc :: gid_t) { self . 0 . sc_egid = gid ; } # [doc = " Gets the current GID."] # [must_use] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub fn get_gid (& self) -> libc :: gid_t { self . 0 . sc_egid } }
};
}
