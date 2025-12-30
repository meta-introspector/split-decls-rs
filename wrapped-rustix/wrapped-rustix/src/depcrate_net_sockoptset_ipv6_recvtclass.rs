// Generated macro for set_ipv6_recvtclass (function)
macro_rules! Depcrate_net_sockoptset_ipv6_recvtclass {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ipv6_recvtclass"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IPV6, IPV6_RECVTCLASS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (any (bsd , linux_like , target_os = "aix" , target_os = "fuchsia" , target_os = "nto"))] # [inline] # [doc (alias = "IPV6_RECVTCLASS")] pub fn set_ipv6_recvtclass < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_ipv6_recvtclass (fd . as_fd () , value) }
};
}
