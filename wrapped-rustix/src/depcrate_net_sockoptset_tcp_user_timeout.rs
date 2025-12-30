// Generated macro for set_tcp_user_timeout (function)
macro_rules! Depcrate_net_sockoptset_tcp_user_timeout {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_user_timeout"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_USER_TIMEOUT, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_USER_TIMEOUT")] pub fn set_tcp_user_timeout < Fd : AsFd > (fd : Fd , value : u32) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_user_timeout (fd . as_fd () , value) }
};
}
