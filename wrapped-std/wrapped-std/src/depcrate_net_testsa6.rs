// Generated macro for sa6 (function)
macro_rules! Depcrate_net_testsa6 {
() => {
// Module: crate::net::test
// Provides: {"sa6"}
// Dependencies: {}
pub fn sa6 (a : Ipv6Addr , p : u16) -> SocketAddr { SocketAddr :: V6 (SocketAddrV6 :: new (a , p , 0 , 0)) }
};
}
