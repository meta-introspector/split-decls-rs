// Generated macro for fd_filestat_get (function)
macro_rules! Depcrate_lib_generatedfd_filestat_get {
() => {
// Module: crate::lib_generated
// Provides: {"fd_filestat_get"}
// Dependencies: {}
# [doc = " Return the attributes of an open file."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The buffer where the file's attributes are stored."] pub unsafe fn fd_filestat_get (fd : Fd) -> Result < Filestat , Errno > { let mut rp0 = MaybeUninit :: < Filestat > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_filestat_get (fd as i32 , rp0 . as_mut_ptr () as i32) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Filestat)) , _ => Err (Errno (ret as u16)) , } }
};
}
