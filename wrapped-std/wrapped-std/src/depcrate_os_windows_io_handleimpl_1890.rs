// Generated macro for impl_1890 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1890 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1890"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: process :: ChildStdout > for OwnedHandle { # [doc = " Takes ownership of a [`ChildStdout`](crate::process::ChildStdout)'s file handle."] # [inline] fn from (child_stdout : crate :: process :: ChildStdout) -> OwnedHandle { unsafe { OwnedHandle :: from_raw_handle (child_stdout . into_raw_handle ()) } } }
};
}
