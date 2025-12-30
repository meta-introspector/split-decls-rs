// Generated macro for impl_501 (impl)
macro_rules! Depcrate_instantimpl_501 {
() => {
// Module: crate::instant
// Provides: {"impl_501"}
// Dependencies: {}
impl Sub for Instant { type Output = Duration ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] fn sub (self , other : Self) -> Self :: Output { match self . 0 . cmp (& other . 0) { Ordering :: Equal => Duration :: ZERO , Ordering :: Greater => (self . 0 - other . 0) . try_into () . expect ("overflow converting `std::time::Duration` to `time::Duration`") , Ordering :: Less => - Duration :: try_from (other . 0 - self . 0) . expect ("overflow converting `std::time::Duration` to `time::Duration`") , } } }
};
}
