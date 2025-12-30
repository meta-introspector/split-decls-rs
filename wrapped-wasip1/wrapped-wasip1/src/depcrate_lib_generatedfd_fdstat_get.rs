// Generated macro for fd_fdstat_get (function)
macro_rules! Depcrate_lib_generatedfd_fdstat_get {
() => {
// Module: crate::lib_generated
// Provides: {"fd_fdstat_get"}
// Dependencies: {}
# [doc = " Get the attributes of a file descriptor."] # [doc = " Note: This returns similar flags to `fsync(fd, F_GETFL)` in POSIX, as well as additional fields."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The buffer where the file descriptor's attributes are stored."] pub unsafe fn fd_fdstat_get (fd : Fd) -> Result < Fdstat , Errno > { let mut rp0 = MaybeUninit :: < Fdstat > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_fdstat_get (fd as i32 , rp0 . as_mut_ptr () as i32) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Fdstat)) , _ => Err (Errno (ret as u16)) , } }
};
}
