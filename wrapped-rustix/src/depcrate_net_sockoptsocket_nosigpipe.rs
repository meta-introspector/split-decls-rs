// Generated macro for socket_nosigpipe (function)
macro_rules! Depcrate_net_sockoptsocket_nosigpipe {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_nosigpipe"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_NOSIGPIPE)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (any (apple , freebsdlike , target_os = "netbsd"))] # [doc (alias = "SO_NOSIGPIPE")] # [inline] pub fn socket_nosigpipe < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_nosigpipe (fd . as_fd ()) }
};
}
