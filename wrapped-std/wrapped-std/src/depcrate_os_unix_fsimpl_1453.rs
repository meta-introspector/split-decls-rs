// Generated macro for impl_1453 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1453 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1453"}
// Dependencies: {}
# [stable (feature = "dir_builder" , since = "1.6.0")] impl DirBuilderExt for fs :: DirBuilder { fn mode (& mut self , mode : u32) -> & mut fs :: DirBuilder { self . as_inner_mut () . set_mode (mode) ; self } }
};
}
