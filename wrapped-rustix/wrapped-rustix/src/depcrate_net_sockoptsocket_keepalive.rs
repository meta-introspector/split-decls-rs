// Generated macro for socket_keepalive (function)
macro_rules! Depcrate_net_sockoptsocket_keepalive {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_keepalive"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_KEEPALIVE)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_KEEPALIVE")] pub fn socket_keepalive < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_keepalive (fd . as_fd ()) }
};
}
