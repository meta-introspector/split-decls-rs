// Generated macro for impl_1990 (impl)
macro_rules! Depcrate_os_windows_processimpl_1990 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_1990"}
// Dependencies: {}
# [stable (feature = "process_extensions" , since = "1.2.0")] impl FromRawHandle for process :: Stdio { unsafe fn from_raw_handle (handle : RawHandle) -> process :: Stdio { let handle = unsafe { sys :: handle :: Handle :: from_raw_handle (handle as * mut _) } ; let io = sys :: process :: Stdio :: Handle (handle) ; process :: Stdio :: from_inner (io) } }
};
}
