// Generated macro for set_permissions_nofollow (function)
macro_rules! Depcrate_sys_fsset_permissions_nofollow {
() => {
// Module: crate::sys::fs
// Provides: {"set_permissions_nofollow"}
// Dependencies: {}
# [cfg (not (unix))] pub fn set_permissions_nofollow (_path : & Path , _perm : crate :: fs :: Permissions) -> io :: Result < () > { crate :: unimplemented ! ("`set_permissions_nofollow` is currently only implemented on Unix platforms") }
};
}
