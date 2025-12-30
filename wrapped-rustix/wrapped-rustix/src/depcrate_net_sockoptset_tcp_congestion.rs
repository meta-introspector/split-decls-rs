// Generated macro for set_tcp_congestion (function)
macro_rules! Depcrate_net_sockoptset_tcp_congestion {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_congestion"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_CONGESTION, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "freebsd" , target_os = "fuchsia" , target_os = "illumos"))] # [inline] # [doc (alias = "TCP_CONGESTION")] pub fn set_tcp_congestion < Fd : AsFd > (fd : Fd , value : & str) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_congestion (fd . as_fd () , value) }
};
}
