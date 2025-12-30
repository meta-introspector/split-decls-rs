// Generated macro for impl_1892 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1892 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1892"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: process :: ChildStderr > for OwnedHandle { # [doc = " Takes ownership of a [`ChildStderr`](crate::process::ChildStderr)'s file handle."] # [inline] fn from (child_stderr : crate :: process :: ChildStderr) -> OwnedHandle { unsafe { OwnedHandle :: from_raw_handle (child_stderr . into_raw_handle ()) } } }
};
}
