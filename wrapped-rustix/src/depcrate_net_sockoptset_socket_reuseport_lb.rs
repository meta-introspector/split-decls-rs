// Generated macro for set_socket_reuseport_lb (function)
macro_rules! Depcrate_net_sockoptset_socket_reuseport_lb {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_reuseport_lb"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_REUSEPORT_LB, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (target_os = "freebsd")] # [inline] # [doc (alias = "SO_REUSEPORT_LB")] pub fn set_socket_reuseport_lb < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_reuseport_lb (fd . as_fd () , value) }
};
}
