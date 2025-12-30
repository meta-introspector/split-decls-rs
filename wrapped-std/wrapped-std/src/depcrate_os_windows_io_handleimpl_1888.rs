// Generated macro for impl_1888 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1888 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1888"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: process :: ChildStdin > for OwnedHandle { # [doc = " Takes ownership of a [`ChildStdin`](crate::process::ChildStdin)'s file handle."] # [inline] fn from (child_stdin : crate :: process :: ChildStdin) -> OwnedHandle { unsafe { OwnedHandle :: from_raw_handle (child_stdin . into_raw_handle ()) } } }
};
}
