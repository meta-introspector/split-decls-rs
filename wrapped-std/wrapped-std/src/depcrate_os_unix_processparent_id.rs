// Generated macro for parent_id (function)
macro_rules! Depcrate_os_unix_processparent_id {
() => {
// Module: crate::os::unix::process
// Provides: {"parent_id"}
// Dependencies: {}
# [doc = " Returns the OS-assigned process identifier associated with this process's parent."] # [must_use] # [stable (feature = "unix_ppid" , since = "1.27.0")] pub fn parent_id () -> u32 { crate :: sys :: os :: getppid () }
};
}
