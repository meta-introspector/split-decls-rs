// Generated macro for set_ipv6_unicast_hops (function)
macro_rules! Depcrate_net_sockoptset_ipv6_unicast_hops {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ipv6_unicast_hops"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_UNICAST_HOPS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_UNICAST_HOPS")] pub fn set_ipv6_unicast_hops < Fd : AsFd > (fd : Fd , value : Option < u8 >) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_unicast_hops (fd . as_fd () , value) }
};
}
