// Generated macro for impl_1920 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1920 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1920"}
// Dependencies: {}
# [stable (feature = "from_raw_os" , since = "1.1.0")] impl FromRawHandle for fs :: File { # [inline] unsafe fn from_raw_handle (handle : RawHandle) -> fs :: File { unsafe { let handle = handle as sys :: c :: HANDLE ; fs :: File :: from_inner (sys :: fs :: File :: from_inner (FromInner :: from_inner (OwnedHandle :: from_raw_handle (handle) ,))) } } }
};
}
