// Generated macro for ipv6_recvtclass (function)
macro_rules! Depcrate_net_sockoptipv6_recvtclass {
() => {
// Module: crate::net::sockopt
// Provides: {"ipv6_recvtclass"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_RECVTCLASS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (any (bsd , linux_like , target_os = "aix" , target_os = "fuchsia" , target_os = "nto"))] # [inline] # [doc (alias = "IPV6_RECVTCLASS")] pub fn ipv6_recvtclass < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ipv6_recvtclass (fd . as_fd ()) }
};
}
