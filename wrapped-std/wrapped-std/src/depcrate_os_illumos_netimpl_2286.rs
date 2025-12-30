// Generated macro for impl_2286 (impl)
macro_rules! Depcrate_os_illumos_netimpl_2286 {
() => {
// Module: crate::os::illumos::net
// Provides: {"impl_2286"}
// Dependencies: {}
# [unstable (feature = "unix_socket_exclbind" , issue = "123481")] impl UnixSocketExt for net :: UnixStream { fn exclbind (& self) -> io :: Result < bool > { self . as_inner () . exclbind () } fn so_exclbind (& self , excl : bool) -> io :: Result < () > { self . as_inner () . set_exclbind (excl) } }
};
}
