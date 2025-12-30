// Generated macro for to_cpath (function)
macro_rules! Depcrate_unix_linux_utilsto_cpath {
() => {
// Module: crate::unix::linux::utils
// Provides: {"to_cpath"}
// Dependencies: {}
# [doc = " Converts a path to a NUL-terminated `Vec<u8>` suitable for use with C functions."] # [cfg (feature = "disk")] pub (crate) fn to_cpath (path : & std :: path :: Path) -> Vec < u8 > { use std :: { ffi :: OsStr , os :: unix :: ffi :: OsStrExt } ; let path_os : & OsStr = path . as_ref () ; let mut cpath = path_os . as_bytes () . to_vec () ; cpath . push (0) ; cpath }
};
}
