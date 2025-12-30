// Generated macro for set_socket_incoming_cpu (function)
macro_rules! Depcrate_net_sockoptset_socket_incoming_cpu {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_incoming_cpu"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_INCOMING_CPU, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (target_os = "linux")] # [inline] # [doc (alias = "SO_INCOMING_CPU")] pub fn set_socket_incoming_cpu < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_incoming_cpu (fd . as_fd () , value) }
};
}
