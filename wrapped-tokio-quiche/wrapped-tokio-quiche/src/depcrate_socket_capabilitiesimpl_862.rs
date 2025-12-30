// Generated macro for impl_862 (impl)
macro_rules! Depcrate_socket_capabilitiesimpl_862 {
() => {
// Module: crate::socket::capabilities
// Provides: {"impl_862"}
// Dependencies: {}
impl SocketCapabilities { # [doc = " Tries to enable all supported sockopts and returns indicators"] # [doc = " of which settings were successfully applied."] # [cfg (target_os = "linux")] pub fn apply_all_and_get_compatibility < S > (socket : & S) -> Self where S : AsFd , { let mut b = SocketCapabilitiesBuilder :: new (socket) ; let _ = b . gso () ; let _ = b . check_udp_drop () ; let _ = b . txtime () ; # [cfg (feature = "perf-quic-listener-metrics")] let _ = b . rxtime () ; let _ = b . gro () ; let _ = b . rcvmark () ; let _ = b . ip_mtu_discover_probe () ; let _ = b . ipv6_mtu_discover_probe () ; if let Ok (true) = b . allows_nonlocal_source () { let _ = b . ipv4_pktinfo () ; let _ = b . ipv4_recvorigdstaddr () ; let _ = b . ipv6_pktinfo () ; let _ = b . ipv6_recvorigdstaddr () ; } b . finish () } }
};
}
