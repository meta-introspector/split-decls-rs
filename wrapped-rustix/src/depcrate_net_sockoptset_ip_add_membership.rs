// Generated macro for set_ip_add_membership (function)
macro_rules! Depcrate_net_sockoptset_ip_add_membership {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_add_membership"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_ADD_MEMBERSHIP, multiaddr, interface)`"] # [doc = ""] # [doc = " This is similar to [`set_ip_add_membership`] but always sets the `ifindex`"] # [doc = " value to zero. See [`set_ip_add_membership_with_ifindex`] instead to also"] # [doc = " give the `ifindex` value."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_ADD_MEMBERSHIP")] pub fn set_ip_add_membership < Fd : AsFd > (fd : Fd , multiaddr : & Ipv4Addr , interface : & Ipv4Addr ,) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_add_membership (fd . as_fd () , multiaddr , interface) }
};
}
