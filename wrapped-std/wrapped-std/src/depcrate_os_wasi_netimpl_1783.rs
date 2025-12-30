// Generated macro for impl_1783 (impl)
macro_rules! Depcrate_os_wasi_netimpl_1783 {
() => {
// Module: crate::os::wasi::net
// Provides: {"impl_1783"}
// Dependencies: {}
impl TcpListenerExt for net :: TcpListener { fn sock_accept (& self , flags : u16) -> io :: Result < u32 > { self . as_inner () . as_inner () . as_inner () . sock_accept (flags) } }
};
}
