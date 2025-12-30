// Generated macro for socket_protocol (function)
macro_rules! Depcrate_net_sockoptsocket_protocol {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_protocol"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_PROTOCOL)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia" , target_os = "openbsd" , target_os = "redox" , target_env = "newlib"))] # [inline] # [doc (alias = "SO_PROTOCOL")] pub fn socket_protocol < Fd : AsFd > (fd : Fd) -> io :: Result < Option < Protocol > > { backend :: net :: sockopt :: socket_protocol (fd . as_fd ()) }
};
}
