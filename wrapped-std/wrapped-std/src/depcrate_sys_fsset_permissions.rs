// Generated macro for set_permissions (function)
macro_rules! Depcrate_sys_fsset_permissions {
() => {
// Module: crate::sys::fs
// Provides: {"set_permissions"}
// Dependencies: {}
pub fn set_permissions (path : & Path , perm : FilePermissions) -> io :: Result < () > { with_native_path (path , & | path | imp :: set_perm (path , perm . clone ())) }
};
}
