// Generated macro for ip_ttl (function)
macro_rules! Depcrate_net_sockoptip_ttl {
() => {
// Module: crate::net::sockopt
// Provides: {"ip_ttl"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_IP, IP_TTL)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [inline] # [doc (alias = "IP_TTL")] pub fn ip_ttl < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: ip_ttl (fd . as_fd ()) }
};
}
