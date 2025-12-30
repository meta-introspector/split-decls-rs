// Generated macro for impl_2285 (impl)
macro_rules! Depcrate_os_illumos_netimpl_2285 {
() => {
// Module: crate::os::illumos::net
// Provides: {"impl_2285"}
// Dependencies: {}
# [unstable (feature = "unix_socket_exclbind" , issue = "123481")] impl UnixSocketExt for net :: UnixDatagram { fn exclbind (& self) -> io :: Result < bool > { self . as_inner () . exclbind () } fn so_exclbind (& self , excl : bool) -> io :: Result < () > { self . as_inner () . set_exclbind (excl) } }
};
}
