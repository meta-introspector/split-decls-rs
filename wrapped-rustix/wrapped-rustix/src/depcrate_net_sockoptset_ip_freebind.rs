// Generated macro for set_ip_freebind (function)
macro_rules! Depcrate_net_sockoptset_ip_freebind {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_freebind"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_FREEBIND, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ipv6_-and-set_ipv6_-functions"] # [cfg (any (linux_kernel , target_os = "fuchsia"))] # [inline] # [doc (alias = "IP_FREEBIND")] pub fn set_ip_freebind < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_freebind (fd . as_fd () , value) }
};
}
