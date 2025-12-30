// Generated macro for impl_2737 (impl)
macro_rules! Depcrate_os_net_linux_ext_socketimpl_2737 {
() => {
// Module: crate::os::net::linux_ext::socket
// Provides: {"impl_2737"}
// Dependencies: {}
# [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] impl UnixSocketExt for net :: UnixDatagram { fn passcred (& self) -> io :: Result < bool > { self . as_inner () . passcred () } fn set_passcred (& self , passcred : bool) -> io :: Result < () > { self . as_inner () . set_passcred (passcred) } }
};
}
