// Generated macro for fd_read (function)
macro_rules! Depcrate_lib_generatedfd_read {
() => {
// Module: crate::lib_generated
// Provides: {"fd_read"}
// Dependencies: {}
# [doc = " Read from a file descriptor."] # [doc = " Note: This is similar to `readv` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `iovs` - List of scatter/gather vectors to which to store data."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The number of bytes read."] pub unsafe fn fd_read (fd : Fd , iovs : IovecArray < '_ >) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_read (fd as i32 , iovs . as_ptr () as i32 , iovs . len () as i32 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
