// Generated macro for tcp_nodelay (function)
macro_rules! Depcrate_net_sockopttcp_nodelay {
() => {
// Module: crate::net::sockopt
// Provides: {"tcp_nodelay"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_NODELAY)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [inline] # [doc (alias = "TCP_NODELAY")] pub fn tcp_nodelay < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: tcp_nodelay (fd . as_fd ()) }
};
}
