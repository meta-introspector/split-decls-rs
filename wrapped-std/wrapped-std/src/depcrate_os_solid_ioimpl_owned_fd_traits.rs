// Generated macro for impl_owned_fd_traits (macro)
macro_rules! Depcrate_os_solid_ioimpl_owned_fd_traits {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_owned_fd_traits"}
// Dependencies: {}
macro_rules ! impl_owned_fd_traits { ($ ($ t : ident) *) => { $ (impl AsFd for net ::$ t { # [inline] fn as_fd (& self) -> BorrowedFd <'_ > { self . as_inner () . socket () . as_fd () } } impl From < net ::$ t > for OwnedFd { # [inline] fn from (socket : net ::$ t) -> OwnedFd { socket . into_inner () . into_socket () . into_inner () } } impl From < OwnedFd > for net ::$ t { # [inline] fn from (owned_fd : OwnedFd) -> Self { Self :: from_inner (FromInner :: from_inner (FromInner :: from_inner (owned_fd))) } }) * } ; }
};
}
