// Generated macro for impl_2745 (impl)
macro_rules! Depcrate_os_net_linux_ext_tcpimpl_2745 {
() => {
// Module: crate::os::net::linux_ext::tcp
// Provides: {"impl_2745"}
// Dependencies: {}
# [stable (feature = "tcp_quickack" , since = "1.89.0")] impl TcpStreamExt for net :: TcpStream { fn set_quickack (& self , quickack : bool) -> io :: Result < () > { self . as_inner () . as_inner () . set_quickack (quickack) } fn quickack (& self) -> io :: Result < bool > { self . as_inner () . as_inner () . quickack () } # [cfg (target_os = "linux")] fn set_deferaccept (& self , accept : u32) -> io :: Result < () > { self . as_inner () . as_inner () . set_deferaccept (accept) } # [cfg (target_os = "linux")] fn deferaccept (& self) -> io :: Result < u32 > { self . as_inner () . as_inner () . deferaccept () } }
};
}
