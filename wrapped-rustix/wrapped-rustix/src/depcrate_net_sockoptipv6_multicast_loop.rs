// Generated macro for ipv6_multicast_loop (function)
macro_rules! Depcrate_net_sockoptipv6_multicast_loop {
() => {
// Module: crate::net::sockopt
// Provides: {"ipv6_multicast_loop"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_MULTICAST_LOOP)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [inline] # [doc (alias = "IPV6_MULTICAST_LOOP")] pub fn ipv6_multicast_loop < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ipv6_multicast_loop (fd . as_fd ()) }
};
}
