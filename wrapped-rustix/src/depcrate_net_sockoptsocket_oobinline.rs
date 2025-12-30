// Generated macro for socket_oobinline (function)
macro_rules! Depcrate_net_sockoptsocket_oobinline {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_oobinline"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_OOBINLINE)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_OOBINLINE")] pub fn socket_oobinline < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_oobinline (fd . as_fd ()) }
};
}
