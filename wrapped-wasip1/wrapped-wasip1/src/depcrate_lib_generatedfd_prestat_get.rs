// Generated macro for fd_prestat_get (function)
macro_rules! Depcrate_lib_generatedfd_prestat_get {
() => {
// Module: crate::lib_generated
// Provides: {"fd_prestat_get"}
// Dependencies: {}
# [doc = " Return a description of the given preopened file descriptor."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The buffer where the description is stored."] pub unsafe fn fd_prestat_get (fd : Fd) -> Result < Prestat , Errno > { let mut rp0 = MaybeUninit :: < Prestat > :: uninit () ; let ret = wasi_snapshot_preview1 :: fd_prestat_get (fd as i32 , rp0 . as_mut_ptr () as i32) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Prestat)) , _ => Err (Errno (ret as u16)) , } }
};
}
