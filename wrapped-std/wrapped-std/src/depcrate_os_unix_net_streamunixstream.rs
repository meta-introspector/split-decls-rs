// Generated macro for UnixStream (struct)
macro_rules! Depcrate_os_unix_net_streamUnixStream {
() => {
// Module: crate::os::unix::net::stream
// Provides: {"UnixStream"}
// Dependencies: {}
# [doc = " A Unix stream socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::os::unix::net::UnixStream;"] # [doc = " use std::io::prelude::*;"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let mut stream = UnixStream::connect(\"/path/to/my/socket\")?;"] # [doc = "     stream.write_all(b\"hello world\")?;"] # [doc = "     let mut response = String::new();"] # [doc = "     stream.read_to_string(&mut response)?;"] # [doc = "     println!(\"{response}\");"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # `SIGPIPE`"] # [doc = ""] # [doc = " Writes to the underlying socket in `SOCK_STREAM` mode are made with `MSG_NOSIGNAL` flag."] # [doc = " This suppresses the emission of the  `SIGPIPE` signal when writing to disconnected socket."] # [doc = " In some cases getting a `SIGPIPE` would trigger process termination."] # [stable (feature = "unix_socket" , since = "1.10.0")] pub struct UnixStream (pub (super) Socket) ;
};
}
