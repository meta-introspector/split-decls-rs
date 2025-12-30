// Generated macro for impl_1814 (impl)
macro_rules! Depcrate_os_windows_fsimpl_1814 {
() => {
// Module: crate::os::windows::fs
// Provides: {"impl_1814"}
// Dependencies: {}
# [stable (feature = "open_options_ext" , since = "1.10.0")] impl OpenOptionsExt for OpenOptions { fn access_mode (& mut self , access : u32) -> & mut OpenOptions { self . as_inner_mut () . access_mode (access) ; self } fn share_mode (& mut self , share : u32) -> & mut OpenOptions { self . as_inner_mut () . share_mode (share) ; self } fn custom_flags (& mut self , flags : u32) -> & mut OpenOptions { self . as_inner_mut () . custom_flags (flags) ; self } fn attributes (& mut self , attributes : u32) -> & mut OpenOptions { self . as_inner_mut () . attributes (attributes) ; self } fn security_qos_flags (& mut self , flags : u32) -> & mut OpenOptions { self . as_inner_mut () . security_qos_flags (flags) ; self } }
};
}
