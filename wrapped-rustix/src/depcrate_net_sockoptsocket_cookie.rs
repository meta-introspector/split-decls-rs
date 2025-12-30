// Generated macro for socket_cookie (function)
macro_rules! Depcrate_net_sockoptsocket_cookie {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_cookie"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_COOKIE)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (target_os = "linux")] # [inline] # [doc (alias = "SO_COOKIE")] pub fn socket_cookie < Fd : AsFd > (fd : Fd) -> io :: Result < u64 > { backend :: net :: sockopt :: socket_cookie (fd . as_fd ()) }
};
}
