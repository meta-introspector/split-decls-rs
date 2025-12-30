// Generated macro for set_socket_oobinline (function)
macro_rules! Depcrate_net_sockoptset_socket_oobinline {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_oobinline"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_OOBINLINE, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_OOBINLINE")] pub fn set_socket_oobinline < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_oobinline (fd . as_fd () , value) }
};
}
