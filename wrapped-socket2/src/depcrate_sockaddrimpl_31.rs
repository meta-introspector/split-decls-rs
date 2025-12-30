// Generated macro for impl_31 (impl)
macro_rules! Depcrate_sockaddrimpl_31 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_31"}
// Dependencies: {}
impl fmt :: Debug for SockAddr { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut f = fmt . debug_struct ("SockAddr") ; # [cfg (any (target_os = "dragonfly" , target_os = "freebsd" , target_os = "haiku" , target_os = "hermit" , target_os = "ios" , target_os = "visionos" , target_os = "macos" , target_os = "netbsd" , target_os = "nto" , target_os = "openbsd" , target_os = "tvos" , target_os = "vxworks" , target_os = "watchos" ,))] f . field ("ss_len" , & self . storage . ss_len) ; f . field ("ss_family" , & self . storage . ss_family) . field ("len" , & self . len) . finish () } }
};
}
