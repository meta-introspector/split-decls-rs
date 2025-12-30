// Generated macro for set_tcp_nodelay (function)
macro_rules! Depcrate_net_sockoptset_tcp_nodelay {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_nodelay"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_NODELAY, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [inline] # [doc (alias = "TCP_NODELAY")] pub fn set_tcp_nodelay < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_nodelay (fd . as_fd () , value) }
};
}
