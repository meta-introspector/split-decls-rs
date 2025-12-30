// Generated macro for impl_57 (impl)
macro_rules! Depcrate_network_reachabilityimpl_57 {
() => {
// Module: crate::network_reachability
// Provides: {"impl_57"}
// Dependencies: {}
impl From < SocketAddr > for SCNetworkReachability { fn from (addr : SocketAddr) -> Self { unsafe { let ptr = SCNetworkReachabilityCreateWithAddress (std :: ptr :: null () , & * to_c_sockaddr (addr)) ; SCNetworkReachability :: wrap_under_create_rule (ptr) } } }
};
}
