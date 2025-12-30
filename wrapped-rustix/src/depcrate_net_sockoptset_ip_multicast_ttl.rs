// Generated macro for set_ip_multicast_ttl (function)
macro_rules! Depcrate_net_sockoptset_ip_multicast_ttl {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_multicast_ttl"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_MULTICAST_TTL, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_MULTICAST_TTL")] pub fn set_ip_multicast_ttl < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_multicast_ttl (fd . as_fd () , value) }
};
}
