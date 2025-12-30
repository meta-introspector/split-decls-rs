// Generated macro for set_permissions_nofollow (function)
macro_rules! Depcrate_fsset_permissions_nofollow {
() => {
// Module: crate::fs
// Provides: {"set_permissions_nofollow"}
// Dependencies: {}
# [doc = " Set the permissions of a file, unless it is a symlink."] # [doc = ""] # [doc = " Note that the non-final path elements are allowed to be symlinks."] # [doc = ""] # [doc = " # Platform-specific behavior"] # [doc = ""] # [doc = " Currently unimplemented on Windows."] # [doc = ""] # [doc = " On Unix platforms, this results in a [`FilesystemLoop`] error if the last element is a symlink."] # [doc = ""] # [doc = " This behavior may change in the future."] # [doc = ""] # [doc = " [`FilesystemLoop`]: crate::io::ErrorKind::FilesystemLoop"] # [doc (alias = "chmod" , alias = "SetFileAttributes")] # [unstable (feature = "set_permissions_nofollow" , issue = "141607")] pub fn set_permissions_nofollow < P : AsRef < Path > > (path : P , perm : Permissions) -> io :: Result < () > { fs_imp :: set_permissions_nofollow (path . as_ref () , perm) }
};
}
