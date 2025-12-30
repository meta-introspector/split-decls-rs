// Generated macro for socket_reuseport (function)
macro_rules! Depcrate_net_sockoptsocket_reuseport {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_reuseport"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_REUSEPORT)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (not (any (solarish , windows , target_os = "cygwin")))] # [inline] # [doc (alias = "SO_REUSEPORT")] pub fn socket_reuseport < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_reuseport (fd . as_fd ()) }
};
}
