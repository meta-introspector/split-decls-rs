// Generated macro for path_link (function)
macro_rules! Depcrate_lib_generatedpath_link {
() => {
// Module: crate::lib_generated
// Provides: {"path_link"}
// Dependencies: {}
# [doc = " Create a hard link."] # [doc = " Note: This is similar to `linkat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `old_flags` - Flags determining the method of how the path is resolved."] # [doc = " * `old_path` - The source path from which to link."] # [doc = " * `new_fd` - The working directory at which the resolution of the new path starts."] # [doc = " * `new_path` - The destination path at which to create the hard link."] pub unsafe fn path_link (old_fd : Fd , old_flags : Lookupflags , old_path : & str , new_fd : Fd , new_path : & str ,) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_link (old_fd as i32 , old_flags as i32 , old_path . as_ptr () as i32 , old_path . len () as i32 , new_fd as i32 , new_path . as_ptr () as i32 , new_path . len () as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
