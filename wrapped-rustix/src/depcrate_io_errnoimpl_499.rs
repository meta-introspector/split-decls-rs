// Generated macro for impl_499 (impl)
macro_rules! Depcrate_io_errnoimpl_499 {
() => {
// Module: crate::io::errno
// Provides: {"impl_499"}
// Dependencies: {}
impl Errno { # [doc = " Shorthand for `std::io::Error::from(self).kind()`."] # [cfg (feature = "std")] # [inline] pub fn kind (self) -> std :: io :: ErrorKind { std :: io :: Error :: from (self) . kind () } }
};
}
