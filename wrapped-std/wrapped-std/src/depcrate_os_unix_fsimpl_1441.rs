// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1441 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1441"}
// Dependencies: {}
# [stable (feature = "fs_ext" , since = "1.1.0")] impl OpenOptionsExt for OpenOptions { fn mode (& mut self , mode : u32) -> & mut OpenOptions { self . as_inner_mut () . mode (mode) ; self } fn custom_flags (& mut self , flags : i32) -> & mut OpenOptions { self . as_inner_mut () . custom_flags (flags) ; self } }
};
}
