// Generated macro for fd_datasync (function)
macro_rules! Depcrate_lib_generatedfd_datasync {
() => {
// Module: crate::lib_generated
// Provides: {"fd_datasync"}
// Dependencies: {}
# [doc = " Synchronize the data of a file to disk."] # [doc = " Note: This is similar to `fdatasync` in POSIX."] pub unsafe fn fd_datasync (fd : Fd) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_datasync (fd as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
