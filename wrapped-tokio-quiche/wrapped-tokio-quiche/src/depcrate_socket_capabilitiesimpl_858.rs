// Generated macro for impl_858 (impl)
macro_rules! Depcrate_socket_capabilitiesimpl_858 {
() => {
// Module: crate::socket::capabilities
// Provides: {"impl_858"}
// Dependencies: {}
# [cfg (target_os = "linux")] impl SetSockOpt for RcvMark { type Val = () ; fn set < F : AsFd > (& self , fd : & F , _val : & Self :: Val) -> nix :: Result < () > { const ENABLE_SOCKOPT : i32 = 1 ; let ret = unsafe { libc :: setsockopt (fd . as_fd () . as_raw_fd () , SOL_SOCKET , SO_RCVMARK , & ENABLE_SOCKOPT as * const c_int as * const c_void , std :: mem :: size_of :: < c_int > () as socklen_t ,) } ; match ret { 0 => Ok (()) , _ => Err (Errno :: last ()) , } } }
};
}
