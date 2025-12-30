// Generated macro for path_rename (function)
macro_rules! Depcrate_lib_generatedpath_rename {
() => {
// Module: crate::lib_generated
// Provides: {"path_rename"}
// Dependencies: {}
# [doc = " Rename a file or directory."] # [doc = " Note: This is similar to `renameat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `old_path` - The source path of the file or directory to rename."] # [doc = " * `new_fd` - The working directory at which the resolution of the new path starts."] # [doc = " * `new_path` - The destination path to which to rename the file or directory."] pub unsafe fn path_rename (fd : Fd , old_path : & str , new_fd : Fd , new_path : & str) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_rename (fd as i32 , old_path . as_ptr () as i32 , old_path . len () as i32 , new_fd as i32 , new_path . as_ptr () as i32 , new_path . len () as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
