// Generated macro for fd_fdstat_set_flags (function)
macro_rules! Depcrate_lib_generatedfd_fdstat_set_flags {
() => {
// Module: crate::lib_generated
// Provides: {"fd_fdstat_set_flags"}
// Dependencies: {}
# [doc = " Adjust the flags associated with a file descriptor."] # [doc = " Note: This is similar to `fcntl(fd, F_SETFL, flags)` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `flags` - The desired values of the file descriptor flags."] pub unsafe fn fd_fdstat_set_flags (fd : Fd , flags : Fdflags) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_fdstat_set_flags (fd as i32 , flags as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
