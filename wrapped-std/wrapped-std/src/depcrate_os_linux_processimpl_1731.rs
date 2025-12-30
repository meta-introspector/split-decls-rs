// Generated macro for impl_1731 (impl)
macro_rules! Depcrate_os_linux_processimpl_1731 {
() => {
// Module: crate::os::linux::process
// Provides: {"impl_1731"}
// Dependencies: {}
impl From < OwnedFd > for PidFd { fn from (fd : OwnedFd) -> Self { Self :: from_inner (InnerPidFd :: from_inner (FileDesc :: from_inner (fd))) } }
};
}
