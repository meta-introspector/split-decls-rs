// Generated macro for set_ip_multicast_if (function)
macro_rules! Depcrate_net_sockoptset_ip_multicast_if {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_multicast_if"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_IF, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_MULTICAST_IF")] pub fn set_ip_multicast_if < Fd : AsFd > (fd : Fd , value : & Ipv4Addr) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_multicast_if (fd . as_fd () , value) }
};
}
