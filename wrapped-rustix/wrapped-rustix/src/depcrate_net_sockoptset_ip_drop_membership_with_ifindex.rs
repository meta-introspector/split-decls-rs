// Generated macro for set_ip_drop_membership_with_ifindex (function)
macro_rules! Depcrate_net_sockoptset_ip_drop_membership_with_ifindex {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_drop_membership_with_ifindex"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_DROP_MEMBERSHIP, multiaddr, interface)`"] # [doc = ""] # [doc = " This is similar to [`set_ip_drop_membership_with_ifindex`] but additionally"] # [doc = " allows a `ifindex` value to be given."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (apple , freebsdlike , linux_like , target_os = "fuchsia" , target_os = "openbsd"))] # [inline] # [doc (alias = "IP_DROP_MEMBERSHIP")] pub fn set_ip_drop_membership_with_ifindex < Fd : AsFd > (fd : Fd , multiaddr : & Ipv4Addr , address : & Ipv4Addr , ifindex : u32 ,) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_drop_membership_with_ifindex (fd . as_fd () , multiaddr , address , ifindex ,) }
};
}
