// Generated macro for impl_961 (impl)
macro_rules! Depcrate_net_socket_addr_anyimpl_961 {
() => {
// Module: crate::net::socket_addr_any
// Provides: {"impl_961"}
// Dependencies: {}
# [allow (clippy :: non_canonical_partial_ord_impl)] impl PartialOrd < Self > for SocketAddrAny { fn partial_cmp (& self , other : & Self) -> Option < core :: cmp :: Ordering > { self . bytes () . partial_cmp (other . bytes ()) } }
};
}
