// Generated macro for set_tcp_keepidle (function)
macro_rules! Depcrate_net_sockoptset_tcp_keepidle {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_keepidle"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_KEEPIDLE, value)`"] # [doc = ""] # [doc = " `TCP_KEEPALIVE` on Apple platforms."] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (not (any (target_os = "haiku" , target_os = "nto" , target_os = "openbsd")))] # [inline] # [doc (alias = "TCP_KEEPIDLE")] pub fn set_tcp_keepidle < Fd : AsFd > (fd : Fd , value : Duration) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_keepidle (fd . as_fd () , value) }
};
}
