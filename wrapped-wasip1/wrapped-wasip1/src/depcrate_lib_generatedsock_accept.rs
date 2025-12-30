// Generated macro for sock_accept (function)
macro_rules! Depcrate_lib_generatedsock_accept {
() => {
// Module: crate::lib_generated
// Provides: {"sock_accept"}
// Dependencies: {}
# [doc = " Accept a new incoming connection."] # [doc = " Note: This is similar to `accept` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `fd` - The listening socket."] # [doc = " * `flags` - The desired values of the file descriptor flags."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " New socket connection"] pub unsafe fn sock_accept (fd : Fd , flags : Fdflags) -> Result < Fd , Errno > { let mut rp0 = MaybeUninit :: < Fd > :: uninit () ; let ret = wasi_snapshot_preview1 :: sock_accept (fd as i32 , flags as i32 , rp0 . as_mut_ptr () as i32) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Fd)) , _ => Err (Errno (ret as u16)) , } }
};
}
