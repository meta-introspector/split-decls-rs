// Generated macro for impl_1991 (impl)
macro_rules! Depcrate_os_windows_processimpl_1991 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_1991"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedHandle > for process :: Stdio { # [doc = " Takes ownership of a handle and returns a [`Stdio`](process::Stdio)"] # [doc = " that can attach a stream to it."] fn from (handle : OwnedHandle) -> process :: Stdio { let handle = sys :: handle :: Handle :: from_inner (handle) ; let io = sys :: process :: Stdio :: Handle (handle) ; process :: Stdio :: from_inner (io) } }
};
}
