// Generated macro for ip_mtu_discover (function)
macro_rules! Depcrate_net_sockoptip_mtu_discover {
() => {
// Module: crate::net::sockopt
// Provides: {"ip_mtu_discover"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IP, IP_MTU_DISCOVER)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "IP_MTU_DISCOVER")] pub fn ip_mtu_discover < Fd : AsFd > (fd : Fd) -> io :: Result < Ipv4PathMtuDiscovery > { backend :: net :: sockopt :: ip_mtu_discover (fd . as_fd ()) }
};
}
