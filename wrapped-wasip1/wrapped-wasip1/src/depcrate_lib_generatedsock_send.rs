// Generated macro for sock_send (function)
macro_rules! Depcrate_lib_generatedsock_send {
() => {
// Module: crate::lib_generated
// Provides: {"sock_send"}
// Dependencies: {}
# [doc = " Send a message on a socket."] # [doc = " Note: This is similar to `send` in POSIX, though it also supports writing"] # [doc = " the data from multiple buffers in the manner of `writev`."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `si_data` - List of scatter/gather vectors to which to retrieve data"] # [doc = " * `si_flags` - Message flags."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " Number of bytes transmitted."] pub unsafe fn sock_send (fd : Fd , si_data : CiovecArray < '_ > , si_flags : Siflags ,) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: sock_send (fd as i32 , si_data . as_ptr () as i32 , si_data . len () as i32 , si_flags as i32 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
