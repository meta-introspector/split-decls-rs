// Generated macro for fd_renumber (function)
macro_rules! Depcrate_lib_generatedfd_renumber {
() => {
// Module: crate::lib_generated
// Provides: {"fd_renumber"}
// Dependencies: {}
# [doc = " Atomically replace a file descriptor by renumbering another file descriptor."] # [doc = " Due to the strong focus on thread safety, this environment does not provide"] # [doc = " a mechanism to duplicate or renumber a file descriptor to an arbitrary"] # [doc = " number, like `dup2()`. This would be prone to race conditions, as an actual"] # [doc = " file descriptor with the same number could be allocated by a different"] # [doc = " thread at the same time."] # [doc = " This function provides a way to atomically renumber file descriptors, which"] # [doc = " would disappear if `dup2()` were to be removed entirely."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `to` - The file descriptor to overwrite."] pub unsafe fn fd_renumber (fd : Fd , to : Fd) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: fd_renumber (fd as i32 , to as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
