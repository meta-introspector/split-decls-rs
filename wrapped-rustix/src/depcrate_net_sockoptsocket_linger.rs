// Generated macro for socket_linger (function)
macro_rules! Depcrate_net_sockoptsocket_linger {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_linger"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_LINGER)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_LINGER")] pub fn socket_linger < Fd : AsFd > (fd : Fd) -> io :: Result < Option < Duration > > { backend :: net :: sockopt :: socket_linger (fd . as_fd ()) }
};
}
