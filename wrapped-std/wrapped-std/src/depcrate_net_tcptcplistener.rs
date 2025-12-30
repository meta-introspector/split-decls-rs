// Generated macro for TcpListener (struct)
macro_rules! Depcrate_net_tcpTcpListener {
() => {
// Module: crate::net::tcp
// Provides: {"TcpListener"}
// Dependencies: {}
# [doc = " A TCP socket server, listening for connections."] # [doc = ""] # [doc = " After creating a `TcpListener` by [`bind`]ing it to a socket address, it listens"] # [doc = " for incoming TCP connections. These can be accepted by calling [`accept`] or by"] # [doc = " iterating over the [`Incoming`] iterator returned by [`incoming`][`TcpListener::incoming`]."] # [doc = ""] # [doc = " The socket will be closed when the value is dropped."] # [doc = ""] # [doc = " The Transmission Control Protocol is specified in [IETF RFC 793]."] # [doc = ""] # [doc = " [`accept`]: TcpListener::accept"] # [doc = " [`bind`]: TcpListener::bind"] # [doc = " [IETF RFC 793]: https://tools.ietf.org/html/rfc793"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::net::{TcpListener, TcpStream};"] # [doc = ""] # [doc = " fn handle_client(stream: TcpStream) {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " fn main() -> std::io::Result<()> {"] # [doc = "     let listener = TcpListener::bind(\"127.0.0.1:80\")?;"] # [doc = ""] # [doc = "     // accept connections and process them serially"] # [doc = "     for stream in listener.incoming() {"] # [doc = "         handle_client(stream?);"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub struct TcpListener (net_imp :: TcpListener) ;
};
}
