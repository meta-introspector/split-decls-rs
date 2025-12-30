// Generated macro for impl_572 (impl)
macro_rules! Depcrate_quic_io_gsoimpl_572 {
() => {
// Module: crate::quic::io::gso
// Provides: {"impl_572"}
// Dependencies: {}
# [cfg (all (target_os = "linux" , not (feature = "fuzzing")))] impl PktInfo { fn make_cmsg (& '_ self) -> ControlMessage < '_ > { match self { Self :: V4 (pkt) => ControlMessage :: Ipv4PacketInfo (pkt) , Self :: V6 (pkt) => ControlMessage :: Ipv6PacketInfo (pkt) , } } fn from_socket_addr (addr : SocketAddr) -> Self { match addr { SocketAddr :: V4 (ipv4) => { let s_addr = u32 :: from_ne_bytes (ipv4 . ip () . octets ()) ; Self :: V4 (libc :: in_pktinfo { ipi_ifindex : 0 , ipi_spec_dst : libc :: in_addr { s_addr } , ipi_addr : libc :: in_addr { s_addr : 0 } , }) } , SocketAddr :: V6 (ipv6) => Self :: V6 (libc :: in6_pktinfo { ipi6_ifindex : 0 , ipi6_addr : libc :: in6_addr { s6_addr : ipv6 . ip () . octets () , } , }) , } } }
};
}
