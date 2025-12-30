// Generated macro for connect_to_server (function)
macro_rules! Depcrate_clientconnect_to_server {
() => {
// Module: crate::client
// Provides: {"connect_to_server"}
// Dependencies: {}
# [doc = " Establish a TCP connection to an sccache server listening on `addr`."] pub fn connect_to_server (addr : & crate :: net :: SocketAddr) -> io :: Result < ServerConnection > { trace ! ("connect_to_server({addr})") ; let conn = crate :: net :: connect (addr) ? ; ServerConnection :: new (conn) }
};
}
