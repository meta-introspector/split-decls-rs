// Generated macro for macro_440 (macro)
macro_rules! Depcrate_fs_xattrmacro_440 {
() => {
// Module: crate::fs::xattr
// Provides: {"macro_440"}
// Dependencies: {}
bitflags ! { # [doc = " `XATTR_*` constants for use with [`setxattr`], and other `*setxattr`"] # [doc = " functions."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct XattrFlags : ffi :: c_uint { # [doc = " `XATTR_CREATE`"] const CREATE = c :: XATTR_CREATE as c :: c_uint ; # [doc = " `XATTR_REPLACE`"] const REPLACE = c :: XATTR_REPLACE as c :: c_uint ; # [doc = " <https://docs.rs/bitflags/*/bitflags/#externally-defined-flags>"] const _ = ! 0 ; } }
};
}
