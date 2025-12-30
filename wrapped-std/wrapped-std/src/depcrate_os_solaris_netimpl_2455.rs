// Generated macro for impl_2455 (impl)
macro_rules! Depcrate_os_solaris_netimpl_2455 {
() => {
// Module: crate::os::solaris::net
// Provides: {"impl_2455"}
// Dependencies: {}
# [unstable (feature = "unix_socket_exclbind" , issue = "123481")] impl UnixSocketExt for net :: UnixDatagram { fn exclbind (& self) -> io :: Result < bool > { self . as_inner () . exclbind () } fn so_exclbind (& self , excl : bool) -> io :: Result < () > { self . as_inner () . set_exclbind (excl) } }
};
}
