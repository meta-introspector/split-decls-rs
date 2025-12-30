// Generated macro for clock_res_get (function)
macro_rules! Depcrate_lib_generatedclock_res_get {
() => {
// Module: crate::lib_generated
// Provides: {"clock_res_get"}
// Dependencies: {}
# [doc = " Return the resolution of a clock."] # [doc = " Implementations are required to provide a non-zero value for supported clocks. For unsupported clocks,"] # [doc = " return `errno::inval`."] # [doc = " Note: This is similar to `clock_getres` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `id` - The clock for which to return the resolution."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The resolution of the clock, or an error if one happened."] pub unsafe fn clock_res_get (id : Clockid) -> Result < Timestamp , Errno > { let mut rp0 = MaybeUninit :: < Timestamp > :: uninit () ; let ret = wasi_snapshot_preview1 :: clock_res_get (id . 0 as i32 , rp0 . as_mut_ptr () as i32) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Timestamp)) , _ => Err (Errno (ret as u16)) , } }
};
}
