// Generated macro for tsa (function)
macro_rules! Depcrate_net_testtsa {
() => {
// Module: crate::net::test
// Provides: {"tsa"}
// Dependencies: {}
pub fn tsa < A : ToSocketAddrs > (a : A) -> Result < Vec < SocketAddr > , String > { match a . to_socket_addrs () { Ok (a) => Ok (a . collect ()) , Err (e) => Err (e . to_string ()) , } }
};
}
