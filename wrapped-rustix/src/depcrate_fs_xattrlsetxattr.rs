// Generated macro for lsetxattr (function)
macro_rules! Depcrate_fs_xattrlsetxattr {
() => {
// Module: crate::fs::xattr
// Provides: {"lsetxattr"}
// Dependencies: {}
# [doc = " `setxattr(path, name, value.as_ptr(), value.len(), flags)`—Set extended"] # [doc = " filesystem attributes, without following symlinks in the last path"] # [doc = " component."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/lsetxattr.2.html"] # [inline] pub fn lsetxattr < P : path :: Arg , Name : path :: Arg > (path : P , name : Name , value : & [u8] , flags : XattrFlags ,) -> io :: Result < () > { path . into_with_c_str (| path | { name . into_with_c_str (| name | backend :: fs :: syscalls :: lsetxattr (path , name , value , flags)) }) }
};
}
