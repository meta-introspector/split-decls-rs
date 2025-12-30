// Generated macro for ipv6_freebind (function)
macro_rules! Depcrate_net_sockoptipv6_freebind {
() => {
// Module: crate::net::sockopt
// Provides: {"ipv6_freebind"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_FREEBIND)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IPV6_FREEBIND")] pub fn ipv6_freebind < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ipv6_freebind (fd . as_fd ()) }
};
}
