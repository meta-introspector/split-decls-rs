// Generated macro for sock_shutdown (function)
macro_rules! Depcrate_lib_generatedsock_shutdown {
() => {
// Module: crate::lib_generated
// Provides: {"sock_shutdown"}
// Dependencies: {}
# [doc = " Shut down socket send and receive channels."] # [doc = " Note: This is similar to `shutdown` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `how` - Which channels on the socket to shut down."] pub unsafe fn sock_shutdown (fd : Fd , how : Sdflags) -> Result < () , Errno > { let ret = wasi_snapshot_preview1 :: sock_shutdown (fd as i32 , how as i32) ; match ret { 0 => Ok (()) , _ => Err (Errno (ret as u16)) , } }
};
}
