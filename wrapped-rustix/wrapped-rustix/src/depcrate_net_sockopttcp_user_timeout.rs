// Generated macro for tcp_user_timeout (function)
macro_rules! Depcrate_net_sockopttcp_user_timeout {
() => {
// Module: crate::net::sockopt
// Provides: {"tcp_user_timeout"}
// Dependencies: {}
# [doc = " `getsockopt(fd, IPPROTO_TCP, TCP_USER_TIMEOUT)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_USER_TIMEOUT")] pub fn tcp_user_timeout < Fd : AsFd > (fd : Fd) -> io :: Result < u32 > { backend :: net :: sockopt :: tcp_user_timeout (fd . as_fd ()) }
};
}
