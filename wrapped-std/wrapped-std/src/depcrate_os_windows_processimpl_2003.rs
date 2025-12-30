// Generated macro for impl_2003 (impl)
macro_rules! Depcrate_os_windows_processimpl_2003 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2003"}
// Dependencies: {}
# [doc = " Creates a `ChildStdout` from the provided `OwnedHandle`."] # [doc = ""] # [doc = " The provided handle must be asynchronous, as reading and"] # [doc = " writing from and to it is implemented using asynchronous APIs."] # [stable (feature = "child_stream_from_fd" , since = "1.74.0")] impl From < OwnedHandle > for process :: ChildStdout { fn from (handle : OwnedHandle) -> process :: ChildStdout { let handle = sys :: handle :: Handle :: from_inner (handle) ; let pipe = sys :: pipe :: AnonPipe :: from_inner (handle) ; process :: ChildStdout :: from_inner (pipe) } }
};
}
