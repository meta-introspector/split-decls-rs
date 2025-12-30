// Generated macro for impl_70 (impl)
macro_rules! Depcrate_durationimpl_70 {
() => {
// Module: crate::duration
// Provides: {"impl_70"}
// Dependencies: {}
impl Sub for Duration { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn sub (self , rhs : Self) -> Self :: Output { self . checked_sub (rhs) . expect ("overflow when subtracting durations") } }
};
}
