// Generated macro for fd_tell (function)
macro_rules! Depcrate_lib_generatedfd_tell {
() => {
// Module: crate::lib_generated
// Provides: {"fd_tell"}
// Dependencies: {}
# [doc = " Return the current offset of a file descriptor."] # [doc = " Note: This is similar to `lseek(fd, 0, SEEK_CUR)` in POSIX."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The current offset of the file descriptor, relative to the start of the file."] pub unsafe fn fd_tell (fd : Fd) -> Result < Filesize , Errno > { let mut rp0 = MaybeUninit :: < Filesize > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_tell (fd as i32 , rp0 . as_mut_ptr () as i32) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Filesize)) , _ => Err (Errno (ret as u16)) , } }
};
}
