// Generated macro for ipv6_tclass (function)
macro_rules! Depcrate_net_sockoptipv6_tclass {
() => {
// Module: crate::net::sockopt
// Provides: {"ipv6_tclass"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IPV6, IPV6_TCLASS)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (not (any (solarish , windows , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "redox" , target_os = "vita")))] # [inline] # [doc (alias = "IPV6_TCLASS")] pub fn ipv6_tclass < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ipv6_tclass (fd . as_fd ()) }
};
}
