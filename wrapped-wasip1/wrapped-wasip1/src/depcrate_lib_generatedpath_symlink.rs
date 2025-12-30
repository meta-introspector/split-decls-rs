// Generated macro for path_symlink (function)
macro_rules! Depcrate_lib_generatedpath_symlink {
() => {
// Module: crate::lib_generated
// Provides: {"path_symlink"}
// Dependencies: {}
# [doc = " Create a symbolic link."] # [doc = " Note: This is similar to `symlinkat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `old_path` - The contents of the symbolic link."] # [doc = " * `new_path` - The destination path at which to create the symbolic link."] pub unsafe fn path_symlink (old_path : & str , fd : Fd , new_path : & str) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_symlink (old_path . as_ptr () as i32 , old_path . len () as i32 , fd as i32 , new_path . as_ptr () as i32 , new_path . len () as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
