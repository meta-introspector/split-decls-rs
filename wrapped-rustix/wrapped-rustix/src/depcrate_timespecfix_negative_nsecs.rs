// Generated macro for fix_negative_nsecs (function)
macro_rules! Depcrate_timespecfix_negative_nsecs {
() => {
// Module: crate::timespec
// Provides: {"fix_negative_nsecs"}
// Dependencies: {}
# [doc = " As described [here], Apple platforms may return a negative nanoseconds"] # [doc = " value in some cases; adjust it so that nanoseconds is always in"] # [doc = " `0..1_000_000_000`."] # [doc = ""] # [doc = " [here]: https://github.com/rust-lang/rust/issues/108277#issuecomment-1787057158"] # [cfg (apple)] # [inline] pub (crate) fn fix_negative_nsecs (mut secs : c :: time_t , mut nsecs : c :: c_long ,) -> (c :: time_t , c :: c_long) { # [cold] fn adjust (secs : & mut c :: time_t , nsecs : c :: c_long) -> c :: c_long { assert ! (nsecs >= - 1_000_000_000) ; assert ! (* secs < 0) ; assert ! (* secs > c :: time_t :: MIN) ; * secs -= 1 ; nsecs + 1_000_000_000 } if nsecs < 0 { nsecs = adjust (& mut secs , nsecs) ; } (secs , nsecs) }
};
}
