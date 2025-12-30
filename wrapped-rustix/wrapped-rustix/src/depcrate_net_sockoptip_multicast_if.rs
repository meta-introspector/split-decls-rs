// Generated macro for ip_multicast_if (function)
macro_rules! Depcrate_net_sockoptip_multicast_if {
() => {
// Module: crate::net::sockopt
// Provides: {"ip_multicast_if"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_IF)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_MULTICAST_IF")] pub fn ip_multicast_if < Fd : AsFd > (fd : Fd) -> io :: Result < Ipv4Addr > { backend :: net :: sockopt :: ip_multicast_if (fd . as_fd ()) }
};
}
