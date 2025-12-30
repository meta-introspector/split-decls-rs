// Generated macro for impl_90 (impl)
macro_rules! Depcrate_durationimpl_90 {
() => {
// Module: crate::duration
// Provides: {"impl_90"}
// Dependencies: {}
impl PartialOrd < StdDuration > for Duration { # [inline] fn partial_cmp (& self , rhs : & StdDuration) -> Option < Ordering > { if rhs . as_secs () > i64 :: MAX . cast_unsigned () { return Some (Ordering :: Less) ; } Some (self . seconds . cmp (& rhs . as_secs () . cast_signed ()) . then_with (| | { self . nanoseconds . get () . cmp (& rhs . subsec_nanos () . cast_signed ()) }) ,) } }
};
}
