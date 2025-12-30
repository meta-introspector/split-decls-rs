// Generated macro for llistxattr (function)
macro_rules! Depcrate_fs_xattrllistxattr {
() => {
// Module: crate::fs::xattr
// Provides: {"llistxattr"}
// Dependencies: {}
# [doc = " `llistxattr(path, list.as_ptr(), list.len())`—List extended filesystem"] # [doc = " attributes, without following symlinks in the last path component."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/llistxattr.2.html"] # [inline] pub fn llistxattr < P : path :: Arg , Buf : Buffer < u8 > > (path : P , mut list : Buf ,) -> io :: Result < Buf :: Output > { path . into_with_c_str (| path | { let len = unsafe { backend :: fs :: syscalls :: llistxattr (path , list . parts_mut ()) ? } ; unsafe { Ok (list . assume_init (len)) } }) }
};
}
