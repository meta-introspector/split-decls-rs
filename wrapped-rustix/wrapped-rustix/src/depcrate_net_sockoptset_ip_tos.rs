// Generated macro for set_ip_tos (function)
macro_rules! Depcrate_net_sockoptset_ip_tos {
() => {
// Module: crate::net::sockopt
// Provides: {"set_ip_tos"}
// Dependencies: {}
# [doc = " `setsockopt(fd, IPPROTO_IP, IP_TOS, value)`"] # [doc = ""] # [doc = " See the [module-level documentation] for more."] # [doc = ""] # [doc = " [module-level documentation]: self#references-for-get_ip_-and-set_ip_-functions"] # [cfg (any (bsd , linux_like , target_os = "aix" , target_os = "fuchsia" , target_os = "haiku" , target_os = "nto" , target_env = "newlib"))] # [inline] # [doc (alias = "IP_TOS")] pub fn set_ip_tos < Fd : AsFd > (fd : Fd , value : u8) -> io :: Result < () > { backend :: net :: sockopt :: set_ip_tos (fd . as_fd () , value) }
};
}
