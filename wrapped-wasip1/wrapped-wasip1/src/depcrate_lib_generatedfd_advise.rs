// Generated macro for fd_advise (function)
macro_rules! Depcrate_lib_generatedfd_advise {
() => {
// Module: crate::lib_generated
// Provides: {"fd_advise"}
// Dependencies: {}
# [doc = " Provide file advisory information on a file descriptor."] # [doc = " Note: This is similar to `posix_fadvise` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `offset` - The offset within the file to which the advisory applies."] # [doc = " * `len` - The length of the region to which the advisory applies."] # [doc = " * `advice` - The advice."] pub unsafe fn fd_advise (fd : Fd , offset : Filesize , len : Filesize , advice : Advice ,) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_advise (fd as i32 , offset as i64 , len as i64 , advice . 0 as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
