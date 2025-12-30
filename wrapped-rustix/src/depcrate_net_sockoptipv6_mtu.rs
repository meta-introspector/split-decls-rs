// Generated macro for ipv6_mtu (function)
macro_rules! Depcrate_net_sockoptipv6_mtu {
() => {
// Module: crate::net::sockopt
// Provides: {"ipv6_mtu"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_MTU)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [cfg (any (linux_kernel , target_os = "cygwin"))] # [doc (alias = "IPV6_MTU")] pub fn ipv6_mtu < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ipv6_mtu (fd . as_fd ()) }
};
}
