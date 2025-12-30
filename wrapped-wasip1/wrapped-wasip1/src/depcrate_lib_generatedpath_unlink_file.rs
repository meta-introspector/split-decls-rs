// Generated macro for path_unlink_file (function)
macro_rules! Depcrate_lib_generatedpath_unlink_file {
() => {
// Module: crate::lib_generated
// Provides: {"path_unlink_file"}
// Dependencies: {}
# [doc = " Unlink a file."] # [doc = " Return `errno::isdir` if the path refers to a directory."] # [doc = " Note: This is similar to `unlinkat(fd, path, 0)` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `path` - The path to a file to unlink."] pub unsafe fn path_unlink_file (fd : Fd , path : & str) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_unlink_file (fd as i32 , path . as_ptr () as i32 , path . len () as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
