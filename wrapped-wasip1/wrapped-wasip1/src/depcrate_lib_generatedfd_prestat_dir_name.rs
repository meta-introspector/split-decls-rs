// Generated macro for fd_prestat_dir_name (function)
macro_rules! Depcrate_lib_generatedfd_prestat_dir_name {
() => {
// Module: crate::lib_generated
// Provides: {"fd_prestat_dir_name"}
// Dependencies: {}
# [doc = " Return a description of the given preopened file descriptor."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `path` - A buffer into which to write the preopened directory name."] pub unsafe fn fd_prestat_dir_name (fd : Fd , path : * mut u8 , path_len : Size) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_prestat_dir_name (fd as i32 , path as i32 , path_len as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
