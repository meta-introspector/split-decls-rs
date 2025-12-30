// Generated macro for impl_2692 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2692 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2692"}
// Dependencies: {}
# [stable (feature = "asfd_ptrs" , since = "1.64.0")] # [doc = " This impl allows implementing traits that require `AsFd` on Arc."] # [doc = " ```"] # [doc = " # #[cfg(any(unix, target_os = \"wasi\"))] mod group_cfg {"] # [doc = " # #[cfg(target_os = \"wasi\")]"] # [doc = " # use std::os::wasi::io::AsFd;"] # [doc = " # #[cfg(unix)]"] # [doc = " # use std::os::unix::io::AsFd;"] # [doc = " use std::net::UdpSocket;"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " trait MyTrait: AsFd {}"] # [doc = " impl MyTrait for Arc<UdpSocket> {}"] # [doc = " impl MyTrait for Box<UdpSocket> {}"] # [doc = " # }"] # [doc = " ```"] impl < T : AsFd + ? Sized > AsFd for crate :: sync :: Arc < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
