// Generated macro for fd_pread (function)
macro_rules! Depcrate_lib_generatedfd_pread {
() => {
// Module: crate::lib_generated
// Provides: {"fd_pread"}
// Dependencies: {}
# [doc = " Read from a file descriptor, without using and updating the file descriptor's offset."] # [doc = " Note: This is similar to `preadv` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `iovs` - List of scatter/gather vectors in which to store data."] # [doc = " * `offset` - The offset within the file at which to read."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The number of bytes read."] pub unsafe fn fd_pread (fd : Fd , iovs : IovecArray < '_ > , offset : Filesize) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_pread (fd as i32 , iovs . as_ptr () as i32 , iovs . len () as i32 , offset as i64 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
