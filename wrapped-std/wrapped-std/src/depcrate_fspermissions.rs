// Generated macro for Permissions (struct)
macro_rules! Depcrate_fsPermissions {
() => {
// Module: crate::fs
// Provides: {"Permissions"}
// Dependencies: {}
# [doc = " Representation of the various permissions on a file."] # [doc = ""] # [doc = " This module only currently provides one bit of information,"] # [doc = " [`Permissions::readonly`], which is exposed on all currently supported"] # [doc = " platforms. Unix-specific functionality, such as mode bits, is available"] # [doc = " through the [`PermissionsExt`] trait."] # [doc = ""] # [doc = " [`PermissionsExt`]: crate::os::unix::fs::PermissionsExt"] # [derive (Clone , PartialEq , Eq , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] # [cfg_attr (not (test) , rustc_diagnostic_item = "FsPermissions")] pub struct Permissions (fs_imp :: FilePermissions) ;
};
}
