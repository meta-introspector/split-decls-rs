// Generated macro for impl_1439 (impl)
macro_rules! Depcrate_os_unix_fsimpl_1439 {
() => {
// Module: crate::os::unix::fs
// Provides: {"impl_1439"}
// Dependencies: {}
# [stable (feature = "fs_ext" , since = "1.1.0")] impl PermissionsExt for Permissions { fn mode (& self) -> u32 { self . as_inner () . mode () } fn set_mode (& mut self , mode : u32) { * self = Permissions :: from_inner (FromInner :: from_inner (mode)) ; } fn from_mode (mode : u32) -> Permissions { Permissions :: from_inner (FromInner :: from_inner (mode)) } }
};
}
