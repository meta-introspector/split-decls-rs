// Generated macro for socket_reuseport_lb (function)
macro_rules! Depcrate_net_sockoptsocket_reuseport_lb {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_reuseport_lb"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_REUSEPORT_LB)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (target_os = "freebsd")] # [inline] # [doc (alias = "SO_REUSEPORT_LB")] pub fn socket_reuseport_lb < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_reuseport_lb (fd . as_fd ()) }
};
}
