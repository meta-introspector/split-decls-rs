// Generated macro for UnixDatagram (struct)
macro_rules! Depcrate_os_unix_net_datagramUnixDatagram {
() => {
// Module: crate::os::unix::net::datagram
// Provides: {"UnixDatagram"}
// Dependencies: {}
# [doc = " A Unix datagram socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixDatagram;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let socket = UnixDatagram::bind(\"/path/to/my/socket\")?;"] # [doc = "     socket.send_to(b\"hello world\", \"/path/to/other/socket\")?;"] # [doc = "     let mut buf = [0; 100];"] # [doc = "     let (count, address) = socket.recv_from(&mut buf)?;"] # [doc = "     println!(\"socket {:?} sent {:?}\", address, &buf[..count]);"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "unix_socket" , since = "1.10.0")] pub struct UnixDatagram (Socket) ;
};
}
