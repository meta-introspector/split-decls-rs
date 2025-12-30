// Generated macro for impl_1735 (impl)
macro_rules! Depcrate_os_linux_processimpl_1735 {
() => {
// Module: crate::os::linux::process
// Provides: {"impl_1735"}
// Dependencies: {}
impl CommandExt for process :: Command { fn create_pidfd (& mut self , val : bool) -> & mut process :: Command { self . as_inner_mut () . create_pidfd (val) ; self } }
};
}
