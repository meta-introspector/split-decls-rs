// Generated macro for impl_1880 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1880 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1880"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedHandle > for fs :: File { # [doc = " Returns a [`File`](fs::File) that takes ownership of the given handle."] # [inline] fn from (owned : OwnedHandle) -> Self { Self :: from_inner (FromInner :: from_inner (FromInner :: from_inner (owned))) } }
};
}
