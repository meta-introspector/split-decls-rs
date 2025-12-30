// Generated macro for impl_1995 (impl)
macro_rules! Depcrate_os_windows_processimpl_1995 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_1995"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < process :: Child > for OwnedHandle { # [doc = " Takes ownership of a [`Child`](process::Child)'s process handle."] fn from (child : process :: Child) -> OwnedHandle { child . into_inner () . into_handle () . into_inner () } }
};
}
