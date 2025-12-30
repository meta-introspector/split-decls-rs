// Generated macro for UnixSocketExt (trait)
macro_rules! Depcrate_os_illumos_netUnixSocketExt {
() => {
// Module: crate::os::illumos::net
// Provides: {"UnixSocketExt"}
// Dependencies: {}
# [doc = " illumos-specific functionality for `AF_UNIX` sockets [`UnixDatagram`]"] # [doc = " and [`UnixStream`]."] # [doc = ""] # [doc = " [`UnixDatagram`]: net::UnixDatagram"] # [doc = " [`UnixStream`]: net::UnixStream"] # [unstable (feature = "unix_socket_exclbind" , issue = "123481")] pub trait UnixSocketExt : Sealed { # [doc = " Enables exclusive binding on the socket."] # [doc = ""] # [doc = " If true and if the socket had been set with `SO_REUSEADDR`,"] # [doc = " it neutralises its effect."] # [doc = " See [`man 3 tcp`](https://docs.oracle.com/cd/E88353_01/html/E37843/setsockopt-3c.html)"] # [unstable (feature = "unix_socket_exclbind" , issue = "123481")] fn so_exclbind (& self , excl : bool) -> io :: Result < () > ; # [doc = " Get the bind exclusivity bind state of the socket."] # [unstable (feature = "unix_socket_exclbind" , issue = "123481")] fn exclbind (& self) -> io :: Result < bool > ; }
};
}
