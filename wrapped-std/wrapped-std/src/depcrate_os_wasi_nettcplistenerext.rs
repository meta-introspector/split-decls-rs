// Generated macro for TcpListenerExt (trait)
macro_rules! Depcrate_os_wasi_netTcpListenerExt {
() => {
// Module: crate::os::wasi::net
// Provides: {"TcpListenerExt"}
// Dependencies: {}
# [doc = " WASI-specific extensions to [`std::net::TcpListener`]."] # [doc = ""] # [doc = " [`std::net::TcpListener`]: crate::net::TcpListener"] pub trait TcpListenerExt { # [doc = " Accept a socket."] # [doc = ""] # [doc = " This corresponds to the `sock_accept` syscall."] fn sock_accept (& self , flags : u16) -> io :: Result < u32 > ; }
};
}
