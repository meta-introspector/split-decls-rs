// Generated macro for socket_passcred (function)
macro_rules! Depcrate_net_sockoptsocket_passcred {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_passcred"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_PASSCRED)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "SO_PASSCRED")] pub fn socket_passcred < Fd : AsFd > (fd : Fd) -> io :: Result < bool > { backend :: net :: sockopt :: socket_passcred (fd . as_fd ()) }
};
}
