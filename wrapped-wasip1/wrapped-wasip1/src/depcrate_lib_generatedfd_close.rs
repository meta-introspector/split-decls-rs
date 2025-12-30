// Generated macro for fd_close (function)
macro_rules! Depcrate_lib_generatedfd_close {
() => {
// Module: crate::lib_generated
// Provides: {"fd_close"}
// Dependencies: {}
# [doc = " Close a file descriptor."] # [doc = " Note: This is similar to `close` in POSIX."] pub unsafe fn fd_close (fd : Fd) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_close (fd as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
