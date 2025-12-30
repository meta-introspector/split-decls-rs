// Generated macro for set_tcp_cork (function)
macro_rules! Depcrate_net_sockoptset_tcp_cork {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_cork"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_CORK, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , solarish , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_CORK")] pub fn set_tcp_cork < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_cork (fd . as_fd () , value) }
};
}
