// Generated macro for next_test_ip6 (function)
macro_rules! Depcrate_net_testnext_test_ip6 {
() => {
// Module: crate::net::test
// Provides: {"next_test_ip6"}
// Dependencies: {}
pub fn next_test_ip6 () -> SocketAddr { let port = PORT . fetch_add (1 , Ordering :: Relaxed) as u16 + BASE_PORT ; SocketAddr :: V6 (SocketAddrV6 :: new (Ipv6Addr :: new (0 , 0 , 0 , 0 , 0 , 0 , 0 , 1) , port , 0 , 0)) }
};
}
