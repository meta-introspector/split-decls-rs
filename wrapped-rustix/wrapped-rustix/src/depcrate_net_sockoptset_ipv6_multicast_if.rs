// Generated macro for set_ipv6_multicast_if (function)
macro_rules! Depcrate_net_sockoptset_ipv6_multicast_if {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ipv6_multicast_if"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_IF, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IPV6_MULTICAST_IF")] pub fn set_ipv6_multicast_if < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_multicast_if (fd . as_fd () , value) }
};
}
