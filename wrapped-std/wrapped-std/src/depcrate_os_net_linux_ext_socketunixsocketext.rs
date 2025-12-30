// Generated macro for UnixSocketExt (trait)
macro_rules! Depcrate_os_net_linux_ext_socketUnixSocketExt {
() => {
// Module: crate::os::net::linux_ext::socket
// Provides: {"UnixSocketExt"}
// Dependencies: {}
# [doc = " Linux-specific functionality for `AF_UNIX` sockets [`UnixDatagram`]"] # [doc = " and [`UnixStream`]."] # [doc = ""] # [doc = " [`UnixDatagram`]: net::UnixDatagram"] # [doc = " [`UnixStream`]: net::UnixStream"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] pub trait UnixSocketExt : Sealed { # [doc = " Query the current setting of socket option `SO_PASSCRED`."] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] fn passcred (& self) -> io :: Result < bool > ; # [doc = " Enable or disable socket option `SO_PASSCRED`."] # [doc = ""] # [doc = " This option enables the credentials of the sending process to be"] # [doc = " received as a control message in [`AncillaryData`]."] # [doc = ""] # [doc = " [`AncillaryData`]: net::AncillaryData"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " #[cfg(target_os = \"linux\")]"] # [doc = " use std::os::linux::net::UnixSocketExt;"] # [doc = " #[cfg(target_os = \"android\")]"] # [doc = " use std::os::android::net::UnixSocketExt;"] # [doc = " use std::os::unix::net::UnixDatagram;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixDatagram::unbound()?;"] # [doc = "     sock.set_passcred(true).expect(\"set_passcred failed\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] fn set_passcred (& self , passcred : bool) -> io :: Result < () > ; }
};
}
