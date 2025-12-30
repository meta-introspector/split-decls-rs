// Generated macro for impl_2176 (impl)
macro_rules! Depcrate_os_freebsd_netimpl_2176 {
() => {
// Module: crate::os::freebsd::net
// Provides: {"impl_2176"}
// Dependencies: {}
# [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl UnixSocketExt for net :: UnixStream { fn local_creds_persistent (& self) -> io :: Result < bool > { self . as_inner () . local_creds_persistent () } fn set_local_creds_persistent (& self , local_creds_persistent : bool) -> io :: Result < () > { self . as_inner () . set_local_creds_persistent (local_creds_persistent) } fn acceptfilter (& self) -> io :: Result < & CStr > { self . as_inner () . acceptfilter () } fn set_acceptfilter (& self , name : & CStr) -> io :: Result < () > { self . as_inner () . set_acceptfilter (name) } }
};
}
