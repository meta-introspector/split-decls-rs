// Generated macro for impl_64 (impl)
macro_rules! Depcrate_durationimpl_64 {
() => {
// Module: crate::duration
// Provides: {"impl_64"}
// Dependencies: {}
impl Add for Duration { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This may panic if an overflow occurs."] # [inline] # [track_caller] fn add (self , rhs : Self) -> Self :: Output { self . checked_add (rhs) . expect ("overflow when adding durations") } }
};
}
