// Generated macro for fd_filestat_set_times (function)
macro_rules! Depcrate_lib_generatedfd_filestat_set_times {
() => {
// Module: crate::lib_generated
// Provides: {"fd_filestat_set_times"}
// Dependencies: {}
# [doc = " Adjust the timestamps of an open file or directory."] # [doc = " Note: This is similar to `futimens` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `atim` - The desired values of the data access timestamp."] # [doc = " * `mtim` - The desired values of the data modification timestamp."] # [doc = " * `fst_flags` - A bitmask indicating which timestamps to adjust."] pub unsafe fn fd_filestat_set_times (fd : Fd , atim : Timestamp , mtim : Timestamp , fst_flags : Fstflags ,) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_filestat_set_times (fd as i32 , atim as i64 , mtim as i64 , fst_flags as i32 ,) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
