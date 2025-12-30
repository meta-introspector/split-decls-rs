// Generated macro for resolve_dst_addr (function)
macro_rules! Depcrate_quic_routerresolve_dst_addr {
() => {
// Module: crate::quic::router
// Provides: {"resolve_dst_addr"}
// Dependencies: {}
# [doc = " Determine if we should store the destination address for a packet, based on"] # [doc = " an address parsed from a"] # [doc = " [`ControlMessageOwned`](nix::sys::socket::ControlMessageOwned)."] # [doc = ""] # [doc = " This is to prevent overriding the destination address if the packet was"] # [doc = " originally addressed to `local`, as that would cause us to incorrectly"] # [doc = " address packets when sending."] # [doc = ""] # [doc = " Returns the parsed address if it should be stored."] # [cfg (target_os = "linux")] fn resolve_dst_addr (local : & SocketAddr , parsed : & SocketAddr ,) -> Option < SocketAddr > { if local != parsed { return Some (* parsed) ; } None }
};
}
