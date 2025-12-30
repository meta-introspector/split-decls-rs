// Generated macro for impl_854 (impl)
macro_rules! Depcrate_socket_capabilitiesimpl_854 {
() => {
// Module: crate::socket::capabilities
// Provides: {"impl_854"}
// Dependencies: {}
# [cfg (target_os = "linux")] impl SetSockOpt for IpMtuDiscoverProbe { type Val = () ; fn set < F : AsFd > (& self , fd : & F , _val : & Self :: Val) -> nix :: Result < () > { let pmtud_mode : c_int = IP_PMTUDISC_PROBE ; let ret = unsafe { libc :: setsockopt (fd . as_fd () . as_raw_fd () , IPPROTO_IP , IP_MTU_DISCOVER , & pmtud_mode as * const c_int as * const c_void , std :: mem :: size_of :: < c_int > () as socklen_t ,) } ; match ret { 0 => Ok (()) , _ => Err (Errno :: last ()) , } } }
};
}
