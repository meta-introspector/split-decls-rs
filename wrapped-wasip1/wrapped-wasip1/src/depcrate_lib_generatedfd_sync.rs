// Generated macro for fd_sync (function)
macro_rules! Depcrate_lib_generatedfd_sync {
() => {
// Module: crate::lib_generated
// Provides: {"fd_sync"}
// Dependencies: {}
# [doc = " Synchronize the data and metadata of a file to disk."] # [doc = " Note: This is similar to `fsync` in POSIX."] pub unsafe fn fd_sync (fd : Fd) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_sync (fd as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
