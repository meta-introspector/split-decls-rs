// Generated macro for clock_time_get (function)
macro_rules! Depcrate_lib_generatedclock_time_get {
() => {
// Module: crate::lib_generated
// Provides: {"clock_time_get"}
// Dependencies: {}
# [doc = " Return the time value of a clock."] # [doc = " Note: This is similar to `clock_gettime` in POSIX."] # [doc = ""] # [doc = " ## Parameters"] # [doc = ""] # [doc = " * `id` - The clock for which to return the time."] # [doc = " * `precision` - The maximum lag (exclusive) that the returned time value may have, compared to its actual value."] # [doc = ""] # [doc = " ## Return"] # [doc = ""] # [doc = " The time value of the clock."] pub unsafe fn clock_time_get (id : Clockid , precision : Timestamp) -> Result < Timestamp , Errno > { let mut rp0 = MaybeUninit :: < Timestamp > :: uninit () ; let ret = wasi_snapshot_preview1 :: clock_time_get (id . 0 as i32 , precision as i64 , rp0 . as_mut_ptr () as i32 ,) ; match ret { 0 => Ok (core :: ptr :: read (rp0 . as_mut_ptr () as i32 as * const Timestamp)) , _ => Err (Errno (ret as u16)) , } }
};
}
