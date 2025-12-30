// Generated macro for linux_imports (module)
macro_rules! Depcrate_socket_capabilitieslinux_imports {
() => {
// Module: crate::socket::capabilities
// Provides: {"linux_imports"}
// Dependencies: {}
# [cfg (target_os = "linux")] mod linux_imports { pub use libc :: c_int ; pub use libc :: c_void ; pub use libc :: sock_txtime ; pub use libc :: socklen_t ; pub use libc :: IPPROTO_IP ; pub use libc :: IPPROTO_IPV6 ; pub use libc :: IPV6_MTU_DISCOVER ; pub use libc :: IPV6_PMTUDISC_PROBE ; pub use libc :: IP_MTU_DISCOVER ; pub use libc :: IP_PMTUDISC_PROBE ; pub use libc :: SOL_SOCKET ; pub use libc :: SO_RCVMARK ; pub use nix :: errno :: Errno ; pub use nix :: sys :: socket :: getsockopt ; pub use nix :: sys :: socket :: setsockopt ; pub use nix :: sys :: socket :: sockopt :: IpFreebind ; pub use nix :: sys :: socket :: sockopt :: IpTransparent ; pub use nix :: sys :: socket :: sockopt :: Ipv4OrigDstAddr ; pub use nix :: sys :: socket :: sockopt :: Ipv4PacketInfo ; pub use nix :: sys :: socket :: sockopt :: Ipv6OrigDstAddr ; pub use nix :: sys :: socket :: sockopt :: Ipv6RecvPacketInfo ; # [cfg (feature = "perf-quic-listener-metrics")] pub use nix :: sys :: socket :: sockopt :: ReceiveTimestampns ; pub use nix :: sys :: socket :: sockopt :: RxqOvfl ; pub use nix :: sys :: socket :: sockopt :: TxTime ; pub use nix :: sys :: socket :: sockopt :: UdpGroSegment ; pub use nix :: sys :: socket :: sockopt :: UdpGsoSegment ; pub use nix :: sys :: socket :: SetSockOpt ; pub use std :: io ; pub use std :: os :: fd :: AsFd ; pub use std :: os :: fd :: AsRawFd ; pub use std :: os :: fd :: BorrowedFd ; }
};
}
