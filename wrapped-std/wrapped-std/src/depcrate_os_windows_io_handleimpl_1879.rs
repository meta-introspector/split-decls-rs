// Generated macro for impl_1879 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1879 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1879"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < fs :: File > for OwnedHandle { # [doc = " Takes ownership of a [`File`](fs::File)'s underlying file handle."] # [inline] fn from (file : fs :: File) -> OwnedHandle { file . into_inner () . into_inner () . into_inner () } }
};
}
