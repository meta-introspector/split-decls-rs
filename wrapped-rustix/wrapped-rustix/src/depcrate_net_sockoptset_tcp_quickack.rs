// Generated macro for set_tcp_quickack (function)
macro_rules! Depcrate_net_sockoptset_tcp_quickack {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_quickack"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_QUICKACK, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_QUICKACK")] pub fn set_tcp_quickack < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_quickack (fd . as_fd () , value) }
};
}
