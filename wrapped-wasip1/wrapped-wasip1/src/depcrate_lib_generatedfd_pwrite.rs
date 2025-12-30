// Generated macro for fd_pwrite (function)
macro_rules! Depcrate_lib_generatedfd_pwrite {
() => {
// Module: crate::lib_generated
// Provides: {"fd_pwrite"}
// Dependencies: {}
# [doc = " Write to a file descriptor, without using and updating the file descriptor's offset."] # [doc = " Note: This is similar to `pwritev` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `iovs` - List of scatter/gather vectors from which to retrieve data."] # [doc = " * `offset` - The offset within the file at which to write."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The number of bytes written."] pub unsafe fn fd_pwrite (fd : Fd , iovs : CiovecArray < '_ > , offset : Filesize) -> Result < Size , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_pwrite (fd as i32 , iovs . as_ptr () as i32 , iovs . len () as i32 , offset as i64 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size)) , _ => Err (Errno (ret as u16)) , } }
};
}
