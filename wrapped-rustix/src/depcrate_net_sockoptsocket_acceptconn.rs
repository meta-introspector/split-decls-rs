// Generated macro for socket_acceptconn (function)
macro_rules! Depcrate_net_sockoptsocket_acceptconn {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_acceptconn"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_ACCEPTCONN)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (not (apple))] # [inline] # [doc (alias = "SO_ACCEPTCONN")] pub fn socket_acceptconn < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_acceptconn (fd . as_fd ()) }
};
}
