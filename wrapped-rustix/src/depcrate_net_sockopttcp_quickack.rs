// Generated macro for tcp_quickack (function)
macro_rules! Depcrate_net_sockopttcp_quickack {
() => {
// Module: crate::net::sockopt
// Provides: {"tcp_quickack"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_QUICKACK)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_QUICKACK")] pub fn tcp_quickack < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: tcp_quickack (fd . as_fd ()) }
};
}
