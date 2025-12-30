// Generated macro for connect_with_retry (function)
macro_rules! Depcrate_clientconnect_with_retry {
() => {
// Module: crate::client
// Provides: {"connect_with_retry"}
// Dependencies: {}
# [doc = " Attempt to establish a TCP connection to an sccache server listening on `addr`."] # [doc = ""] # [doc = " If the connection fails, retry a few times."] pub fn connect_with_retry (addr : & crate :: net :: SocketAddr) -> io :: Result < ServerConnection > { trace ! ("connect_with_retry({addr})") ; let backoff = backon :: ConstantBuilder :: default () . with_delay (std :: time :: Duration :: from_millis (500)) . with_max_times (10) ; match (| | connect_to_server (addr)) . retry (backoff) . call () { Ok (conn) => Ok (conn) , Err (e) => Err (io :: Error :: new (io :: ErrorKind :: TimedOut , format ! ("Connection to server timed out: {:?}" , e) ,)) , } }
};
}
