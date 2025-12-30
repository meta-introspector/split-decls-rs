// Generated macro for set_socket_keepalive (function)
macro_rules! Depcrate_net_sockoptset_socket_keepalive {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_keepalive"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_KEEPALIVE, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_KEEPALIVE")] pub fn set_socket_keepalive < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_keepalive (fd . as_fd () , value) }
};
}
