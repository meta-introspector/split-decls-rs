// Generated macro for next_test_ip4 (function)
macro_rules! Depcrate_net_testnext_test_ip4 {
() => {
// Module: crate::net::test
// Provides: {"next_test_ip4"}
// Dependencies: {}
pub fn next_test_ip4 () -> SocketAddr { let port = PORT . fetch_add (1 , Ordering :: Relaxed) as u16 + BASE_PORT ; SocketAddr :: V4 (SocketAddrV4 :: new (Ipv4Addr :: new (127 , 0 , 0 , 1) , port)) }
};
}
