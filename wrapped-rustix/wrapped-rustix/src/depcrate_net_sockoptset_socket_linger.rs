// Generated macro for set_socket_linger (function)
macro_rules! Depcrate_net_sockoptset_socket_linger {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_linger"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_LINGER, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_LINGER")] pub fn set_socket_linger < Fd : AsFd > (fd : Fd , value : Option < Duration >) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_linger (fd . as_fd () , value) }
};
}
