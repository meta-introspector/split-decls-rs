// Generated macro for fd_allocate (function)
macro_rules! Depcrate_lib_generatedfd_allocate {
() => {
// Module: crate::lib_generated
// Provides: {"fd_allocate"}
// Dependencies: {}
# [doc = " Force the allocation of space in a file."] # [doc = " Note: This is similar to `posix_fallocate` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `offset` - The offset at which to start the allocation."] # [doc = " * `len` - The length of the area that is allocated."] pub unsafe fn fd_allocate (fd : Fd , offset : Filesize , len : Filesize) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_allocate (fd as i32 , offset as i64 , len as i64) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
