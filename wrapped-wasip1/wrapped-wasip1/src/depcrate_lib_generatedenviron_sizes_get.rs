// Generated macro for environ_sizes_get (function)
macro_rules! Depcrate_lib_generatedenviron_sizes_get {
() => {
// Module: crate::lib_generated
// Provides: {"environ_sizes_get"}
// Dependencies: {}
# [doc = " Return environment variable data sizes."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " Returns the number of environment variable arguments and the size of the"] # [doc = " environment variable data."] pub unsafe fn environ_sizes_get () -> Result < (Size , Size) , Errno > { let mut rp0 = MaybeUninit :: < Size > :: uninit () ; let mut rp1 = MaybeUninit :: < Size > :: uninit () ; let ret = wasi_snapshot_preview1 :: environ_sizes_get (rp0 . as_mut_ptr () as i32 , rp1 . as_mut_ptr () as i32) ; match ret { 0 => Ok ((core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Size) , core :: ptr :: read (rp1 . as_mut_ptr () as i32 as * const Size) ,)) , _ => Err (Errno (ret as u16)) , } }
};
}
