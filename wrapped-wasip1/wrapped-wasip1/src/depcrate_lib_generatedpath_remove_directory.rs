// Generated macro for path_remove_directory (function)
macro_rules! Depcrate_lib_generatedpath_remove_directory {
() => {
// Module: crate::lib_generated
// Provides: {"path_remove_directory"}
// Dependencies: {}
# [doc = " Remove a directory."] # [doc = " Return `errno::notempty` if the directory is not empty."] # [doc = " Note: This is similar to `unlinkat(fd, path, AT_REMOVEDIR)` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `path` - The path to a directory to remove."] pub unsafe fn path_remove_directory (fd : Fd , path : & str) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_remove_directory (fd as i32 , path . as_ptr () as i32 , path . len () as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
