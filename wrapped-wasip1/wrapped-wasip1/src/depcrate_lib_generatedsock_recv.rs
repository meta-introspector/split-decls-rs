// Generated macro for sock_recv (function)
macro_rules! Depcrate_lib_generatedsock_recv {
() => {
// Module: crate::lib_generated
// Provides: {"sock_recv"}
// Dependencies: {}
# [doc = " Receive a message from a socket."] # [doc = " Note: This is similar to `recv` in POSIX, though it also supports reading"] # [doc = " the data into multiple buffers in the manner of `readv`."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `ri_data` - List of scatter/gather vectors to which to store data."] # [doc = " * `ri_flags` - Message flags."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " Number of bytes stored in ri_data and message flags."] pub unsafe fn sock_recv (fd : Fd , ri_data : IovecArray < '_ > , ri_flags : Riflags ,) -> Result < (Size , Roflags) , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let mut rp1 = MaybeUninit :: < Roflags > :: uninit () ; let ret = wasi_snapshot_preview1 :: sock_recv (fd as i32 , ri_data . as_ptr () as i32 , ri_data . len () as i32 , ri_flags as i32 , rp0 . as_mut_ptr () as i32 , rp1 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok ((core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size) , core :: ptr :: read (rp1 . as_mut_ptr () as i32 as * const Roflags) ,)) , _ => Err (Errno (ret as u16)) , } }
};
}
