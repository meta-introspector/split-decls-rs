// Generated macro for fd_seek (function)
macro_rules! Depcrate_lib_generatedfd_seek {
() => {
// Module: crate::lib_generated
// Provides: {"fd_seek"}
// Dependencies: {}
# [doc = " Move the offset of a file descriptor."] # [doc = " Note: This is similar to `lseek` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `offset` - The number of bytes to move."] # [doc = " * `whence` - The base from which the offset is relative."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The new offset of the file descriptor, relative to the start of the file."] pub unsafe fn fd_seek (fd : Fd , offset : Filedelta , whence : Whence) -> Result < Filesize , Errno > { let mut rp0 = MaybeUninit :: < Filesize > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_seek (fd as i32 , offset , whence . 0 as i32 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Filesize)) , _ => Err (Errno (ret as u16)) , } }
};
}
