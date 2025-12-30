// Generated macro for socket_reuseaddr (function)
macro_rules! Depcrate_net_sockoptsocket_reuseaddr {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_reuseaddr"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_REUSEADDR)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_REUSEADDR")] pub fn socket_reuseaddr < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_reuseaddr (fd . as_fd ()) }
};
}
