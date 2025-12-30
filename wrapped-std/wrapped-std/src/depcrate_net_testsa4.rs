// Generated macro for sa4 (function)
macro_rules! Depcrate_net_testsa4 {
() => {
// Module: crate::net::test
// Provides: {"sa4"}
// Dependencies: {}
pub fn sa4 (a : Ipv4Addr , p : u16) -> SocketAddr { SocketAddr :: V4 (SocketAddrV4 :: new (a , p)) }
};
}
