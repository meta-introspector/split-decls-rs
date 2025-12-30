// Generated macro for impl_1963 (impl)
macro_rules! Depcrate_os_windows_io_socketimpl_1963 {
() => {
// Module: crate::os::windows::io::socket
// Provides: {"impl_1963"}
// Dependencies: {}
# [stable (feature = "as_windows_ptrs" , since = "1.71.0")] # [doc = " This impl allows implementing traits that require `AsSocket` on Arc."] # [doc = " ```"] # [doc = " # #[cfg(windows)] mod group_cfg {"] # [doc = " # use std::os::windows::io::AsSocket;"] # [doc = " use std::net::UdpSocket;"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " trait MyTrait: AsSocket {}"] # [doc = " impl MyTrait for Arc<UdpSocket> {}"] # [doc = " impl MyTrait for Box<UdpSocket> {}"] # [doc = " # }"] # [doc = " ```"] impl < T : AsSocket > AsSocket for crate :: sync :: Arc < T > { # [inline] fn as_socket (& self) -> BorrowedSocket < '_ > { (* * self) . as_socket () } }
};
}
