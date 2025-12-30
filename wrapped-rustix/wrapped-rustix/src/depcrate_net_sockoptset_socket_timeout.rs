// Generated macro for set_socket_timeout (function)
macro_rules! Depcrate_net_sockoptset_socket_timeout {
() => {
// Module: crate::net::sockopt
// Provides: {"set_socket_timeout"}
// Dependencies: {}
# [doc = " `setsockopt(fd, SOL_SOCKET, id, value)`—Set the sending or receiving"] # [doc = " timeout."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [inline] # [doc (alias = "SO_RCVTIMEO")] # [doc (alias = "SO_SNDTIMEO")] pub fn set_socket_timeout < Fd : AsFd > (fd : Fd , id : Timeout , value : Option < Duration > ,) -> io :: Result < () > { backend :: net :: sockopt :: set_socket_timeout (fd . as_fd () , id , value) }
};
}
