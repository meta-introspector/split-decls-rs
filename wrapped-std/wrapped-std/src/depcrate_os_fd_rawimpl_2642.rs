// Generated macro for impl_2642 (impl)
macro_rules! Depcrate_os_fd_rawimpl_2642 {
() => {
// Module: crate::os::fd::raw
// Provides: {"impl_2642"}
// Dependencies: {}
# [doc = " This impl allows implementing traits that require `AsRawFd` on Arc."] # [doc = " ```"] # [doc = " # #[cfg(any(unix, target_os = \"wasi\"))] mod group_cfg {"] # [doc = " # #[cfg(target_os = \"wasi\")]"] # [doc = " # use std::os::wasi::io::AsRawFd;"] # [doc = " # #[cfg(unix)]"] # [doc = " # use std::os::unix::io::AsRawFd;"] # [doc = " use std::net::UdpSocket;"] # [doc = " use std::sync::Arc;"] # [doc = " trait MyTrait: AsRawFd {"] # [doc = " }"] # [doc = " impl MyTrait for Arc<UdpSocket> {}"] # [doc = " impl MyTrait for Box<UdpSocket> {}"] # [doc = " # }"] # [doc = " ```"] # [stable (feature = "asrawfd_ptrs" , since = "1.63.0")] impl < T : AsRawFd > AsRawFd for crate :: sync :: Arc < T > { # [inline] fn as_raw_fd (& self) -> RawFd { (* * self) . as_raw_fd () } }
};
}
