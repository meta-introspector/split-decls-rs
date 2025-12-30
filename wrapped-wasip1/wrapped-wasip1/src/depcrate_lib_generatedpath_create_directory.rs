// Generated macro for path_create_directory (function)
macro_rules! Depcrate_lib_generatedpath_create_directory {
() => {
// Module: crate::lib_generated
// Provides: {"path_create_directory"}
// Dependencies: {}
# [doc = " Create a directory."] # [doc = " Note: This is similar to `mkdirat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `path` - The path at which to create the directory."] pub unsafe fn path_create_directory (fd : Fd , path : & str) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_create_directory (fd as i32 , path . as_ptr () as i32 , path . len () as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
