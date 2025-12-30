// Generated macro for MapAction (enum)
macro_rules! Depcrate_hypervisor_vspaceMapAction {
() => {
// Module: crate::hypervisor::vspace
// Provides: {"MapAction"}
// Dependencies: {}
# [doc = " Mapping rights to give to address translation."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] # [allow (unused)] pub enum MapAction { # [doc = " Don't map"] None , # [doc = " Map region read-only."] ReadUser , # [doc = " Map region read-only for kernel."] ReadKernel , # [doc = " Map region read-write."] ReadWriteUser , # [doc = " Map region read-write for kernel."] ReadWriteKernel , # [doc = " Map region read-executable."] ReadExecuteUser , # [doc = " Map region read-executable for kernel."] ReadExecuteKernel , # [doc = " Map region read-write-executable."] ReadWriteExecuteUser , # [doc = " Map region read-write-executable for kernel."] ReadWriteExecuteKernel , }
};
}
