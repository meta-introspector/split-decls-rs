// Generated macro for set_tcp_thin_linear_timeouts (function)
macro_rules! Depcrate_net_sockoptset_tcp_thin_linear_timeouts {
() => {
// Module: crate::net::sockopt
// Provides: {"set_tcp_thin_linear_timeouts"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_TCP, TCP_THIN_LINEAR_TIMEOUTS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_tcp_-and-set_tcp_-functions"] # [cfg (any (linux_like , target_os = "fuchsia"))] # [inline] # [doc (alias = "TCP_THIN_LINEAR_TIMEOUTS")] pub fn set_tcp_thin_linear_timeouts < Fd : AsFd > (fd : Fd , value : bool) -> io :: Result < () > { backend :: net :: sockopt :: set_tcp_thin_linear_timeouts (fd . as_fd () , value) }
};
}
