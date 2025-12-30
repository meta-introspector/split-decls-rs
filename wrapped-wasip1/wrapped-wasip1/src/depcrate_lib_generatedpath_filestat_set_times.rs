// Generated macro for path_filestat_set_times (function)
macro_rules! Depcrate_lib_generatedpath_filestat_set_times {
() => {
// Module: crate::lib_generated
// Provides: {"path_filestat_set_times"}
// Dependencies: {}
# [doc = " Adjust the timestamps of a file or directory."] # [doc = " Note: This is similar to `utimensat` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `flags` - Flags determining the method of how the path is resolved."] # [doc = " * `path` - The path of the file or directory to operate on."] # [doc = " * `atim` - The desired values of the data access timestamp."] # [doc = " * `mtim` - The desired values of the data modification timestamp."] # [doc = " * `fst_flags` - A bitmask indicating which timestamps to adjust."] pub unsafe fn path_filestat_set_times (fd : Fd , flags : Lookupflags , path : & str , atim : Timestamp , mtim : Timestamp , fst_flags : Fstflags ,) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: path_filestat_set_times (fd as i32 , flags as i32 , path . as_ptr () as i32 , path . len () as i32 , atim as i64 , mtim as i64 , fst_flags as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
