// Generated macro for set_ipv6_freebind (function)
macro_rules! Depcrate_net_sockoptset_ipv6_freebind {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ipv6_freebind"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_FREEBIND, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IPV6_FREEBIND")] pub fn set_ipv6_freebind < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_freebind (fd . as_fd () , value) }
};
}
