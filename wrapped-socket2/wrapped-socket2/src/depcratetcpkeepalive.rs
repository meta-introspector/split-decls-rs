// Generated macro for TcpKeepalive (struct)
macro_rules! DepcrateTcpKeepalive {
() => {
// Module: crate
// Provides: {"TcpKeepalive"}
// Dependencies: {}
# [doc = " Configures a socket's TCP keepalive parameters."] # [doc = ""] # [doc = " See [`Socket::set_tcp_keepalive`]."] # [derive (Debug , Clone)] pub struct TcpKeepalive { # [cfg_attr (any (target_os = "openbsd" , target_os = "haiku" , target_os = "vita") , allow (dead_code))] time : Option < Duration > , # [cfg (not (any (target_os = "openbsd" , target_os = "redox" , target_os = "solaris" , target_os = "nto" , target_os = "espidf" , target_os = "vita" , target_os = "haiku" ,)))] interval : Option < Duration > , # [cfg (not (any (target_os = "openbsd" , target_os = "redox" , target_os = "solaris" , target_os = "nto" , target_os = "espidf" , target_os = "vita" , target_os = "haiku" ,)))] retries : Option < u32 > , }
};
}
