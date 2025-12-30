// Generated macro for set_socket_send_buffer_size (function)
macro_rules! Depcrate_net_sockoptset_socket_send_buffer_size {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_send_buffer_size"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, SO_SNDBUF, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_SNDBUF")] pub fn set_socket_send_buffer_size < Fd : AsFd > (fd : Fd , value : usize) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_send_buffer_size (fd . as_fd () , value) }
};
}
