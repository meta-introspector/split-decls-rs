// Generated macro for ip_multicast_loop (function)
macro_rules! Depcrate_net_sockoptip_multicast_loop {
() => {
// Module: crate::net::sockopt
// Provides: {"ip_multicast_loop"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IP, IP_MULTICAST_LOOP)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_MULTICAST_LOOP")] pub fn ip_multicast_loop < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: ip_multicast_loop (fd . as_fd ()) }
};
}
