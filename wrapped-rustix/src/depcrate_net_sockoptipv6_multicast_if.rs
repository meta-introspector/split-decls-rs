// Generated macro for ipv6_multicast_if (function)
macro_rules! Depcrate_net_sockoptipv6_multicast_if {
() => {
// Module: crate::net::sockopt
// Provides: {"ipv6_multicast_if"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_IF)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IPV6_MULTICAST_IF")] pub fn ipv6_multicast_if < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ipv6_multicast_if (fd . as_fd ()) }
};
}
