// Generated macro for impl_1872 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1872 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1872"}
// Dependencies: {}
# [stable (feature = "as_windows_ptrs" , since = "1.71.0")] # [doc = " This impl allows implementing traits that require `AsHandle` on Arc."] # [doc = " ```"] # [doc = " # #[cfg(windows)] mod group_cfg {"] # [doc = " # use std::os::windows::io::AsHandle;"] # [doc = " use std::fs::File;"] # [doc = " use std::sync::Arc;"] # [doc = ""] # [doc = " trait MyTrait: AsHandle {}"] # [doc = " impl MyTrait for Arc<File> {}"] # [doc = " impl MyTrait for Box<File> {}"] # [doc = " # }"] # [doc = " ```"] impl < T : AsHandle + ? Sized > AsHandle for crate :: sync :: Arc < T > { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { (* * self) . as_handle () } }
};
}
