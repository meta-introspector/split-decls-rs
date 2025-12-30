// Generated macro for SocketAddr (struct)
macro_rules! Depcrate_os_unix_net_addrSocketAddr {
() => {
// Module: crate::os::unix::net::addr
// Provides: {"SocketAddr"}
// Dependencies: {}
# [doc = " An address associated with a Unix socket."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::os::unix::net::UnixListener;"] # [doc = ""] # [doc = " let socket = match UnixListener::bind(\"/tmp/sock\") {"] # [doc = "     Ok(sock) => sock,"] # [doc = "     Err(e) => {"] # [doc = "         println!(\"Couldn't bind: {e:?}\");"] # [doc = "         return"] # [doc = "     }"] # [doc = " };"] # [doc = " let addr = socket.local_addr().expect(\"Couldn't get local address\");"] # [doc = " ```"] # [derive (Clone)] # [stable (feature = "unix_socket" , since = "1.10.0")] pub struct SocketAddr { pub (super) addr : libc :: sockaddr_un , pub (super) len : libc :: socklen_t , }
};
}
