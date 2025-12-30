// Generated macro for socket_domain (function)
macro_rules! Depcrate_net_sockoptsocket_domain {
() => {
// Module: crate::net::sockopt
// Provides: {"socket_domain"}
// Dependencies: {}
# [doc = " `getsockopt(fd, SOL_SOCKET, SO_DOMAIN)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_socket_-and-set_socket_-functions"] # [cfg (not (any (apple , windows , target_os = "aix" , target_os = "cygwin" , target_os = "dragonfly" , target_os = "emscripten" , target_os = "espidf" , target_os = "haiku" , target_os = "horizon" , target_os = "hurd" , target_os = "netbsd" , target_os = "nto" , target_os = "vita" ,)))] # [inline] # [doc (alias = "SO_DOMAIN")] pub fn socket_domain < Fd : AsFd > (fd : Fd) -> io :: Result < AddressFamily > { backend :: net :: sockopt :: socket_domain (fd . as_fd ()) }
};
}
