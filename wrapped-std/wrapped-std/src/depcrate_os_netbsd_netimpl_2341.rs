// Generated macro for impl_2341 (impl)
macro_rules! Depcrate_os_netbsd_netimpl_2341 {
() => {
// Module: crate::os::netbsd::net
// Provides: {"impl_2341"}
// Dependencies: {}
# [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl UnixSocketExt for net :: UnixStream { fn local_creds (& self) -> io :: Result < bool > { self . as_inner () . local_creds () } fn set_local_creds (& self , local_creds : bool) -> io :: Result < () > { self . as_inner () . set_local_creds (local_creds) } fn acceptfilter (& self) -> io :: Result < & CStr > { self . as_inner () . acceptfilter () } fn set_acceptfilter (& self , name : & CStr) -> io :: Result < () > { self . as_inner () . set_acceptfilter (name) } }
};
}
