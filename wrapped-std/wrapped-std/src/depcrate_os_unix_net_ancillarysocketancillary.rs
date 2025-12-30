// Generated macro for SocketAncillary (struct)
macro_rules! Depcrate_os_unix_net_ancillarySocketAncillary {
() => {
// Module: crate::os::unix::net::ancillary
// Provides: {"SocketAncillary"}
// Dependencies: {}
# [doc = " A Unix socket Ancillary data struct."] # [doc = ""] # [doc = " # Example"] # [doc = " ```no_run"] # [doc = " #![feature(unix_socket_ancillary_data)]"] # [doc = " use std::os::unix::net::{UnixStream, SocketAncillary, AncillaryData};"] # [doc = " use std::io::IoSliceMut;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let sock = UnixStream::connect(\"/tmp/sock\")?;"] # [doc = ""] # [doc = "     let mut fds = [0; 8];"] # [doc = "     let mut ancillary_buffer = [0; 128];"] # [doc = "     let mut ancillary = SocketAncillary::new(&mut ancillary_buffer[..]);"] # [doc = ""] # [doc = "     let mut buf = [1; 8];"] # [doc = "     let mut bufs = &mut [IoSliceMut::new(&mut buf[..])][..];"] # [doc = "     sock.recv_vectored_with_ancillary(bufs, &mut ancillary)?;"] # [doc = ""] # [doc = "     for ancillary_result in ancillary.messages() {"] # [doc = "         if let AncillaryData::ScmRights(scm_rights) = ancillary_result.unwrap() {"] # [doc = "             for fd in scm_rights {"] # [doc = "                 println!(\"receive file descriptor: {fd}\");"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [unstable (feature = "unix_socket_ancillary_data" , issue = "76915")] # [derive (Debug)] pub struct SocketAncillary < 'a > { buffer : & 'a mut [u8] , length : usize , truncated : bool , }
};
}
