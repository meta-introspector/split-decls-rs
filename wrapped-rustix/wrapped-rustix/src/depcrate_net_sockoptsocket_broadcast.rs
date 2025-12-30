// Generated macro for socket_broadcast (function)
macro_rules! Depcrate_net_sockoptsocket_broadcast {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_broadcast"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_BROADCAST)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_BROADCAST")] pub fn socket_broadcast < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_broadcast (fd . as_fd ()) }
};
}
