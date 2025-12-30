// Generated macro for set_ip_drop_source_membership (function)
macro_rules! Depcrate_net_sockoptset_ip_drop_source_membership {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_drop_source_membership"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_DROP_SOURCE_MEMBERSHIP, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (apple , freebsdlike , linux_like , solarish , target_os = "aix"))] # [inline] # [doc (alias = "IP_DROP_SOURCE_MEMBERSHIP")] pub fn set_ip_drop_source_membership < Fd : AsFd > (fd : Fd , multiaddr : & Ipv4Addr , interface : & Ipv4Addr , sourceaddr : & Ipv4Addr ,) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_drop_source_membership (fd . as_fd () , multiaddr , interface , sourceaddr ,) }
};
}
