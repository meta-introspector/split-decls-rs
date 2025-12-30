// Generated macro for impl_2503 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2503 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2503"}
// Dependencies: {}
# [doc = " This impl allows implementing traits that require `AsFd` on Arc."] # [doc = " ```"] # [doc = " # #[cfg(target_os = \"solid_asp3\")] mod group_cfg {"] # [doc = " # use std::os::solid::io::AsFd;"] # [doc = " use std::net::UdpSocket;"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " trait MyTrait: AsFd {}"] # [doc = " impl MyTrait for Arc<UdpSocket> {}"] # [doc = " impl MyTrait for Box<UdpSocket> {}"] # [doc = " # }"] # [doc = " ```"] impl < T : AsFd > AsFd for crate :: sync :: Arc < T > { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { (* * self) . as_fd () } }
};
}
