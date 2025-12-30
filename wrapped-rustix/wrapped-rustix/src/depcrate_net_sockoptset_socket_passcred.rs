// Generated macro for set_socket_passcred (function)
macro_rules! Depcrate_net_sockoptset_socket_passcred {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_passcred"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_PASSCRED, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (linux_kernel)] # [inline] # [doc (alias = "SO_PASSCRED")] pub fn set_socket_passcred < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_passcred (fd . as_fd () , value) }
};
}
