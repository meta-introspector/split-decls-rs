// Generated macro for impl_231 (impl)
macro_rules! Depcrate_hedge_rotating_histogramimpl_231 {
() => {
// Module: crate::hedge::rotating_histogram
// Provides: {"impl_231"}
// Dependencies: {}
impl RotatingHistogram { pub fn new (period : Duration) -> RotatingHistogram { RotatingHistogram { read : Histogram :: < u64 > :: new (3) . expect ("Invalid histogram params") , write : Histogram :: < u64 > :: new (3) . expect ("Invalid histogram params") , last_rotation : Instant :: now () , period , } } pub fn read (& mut self) -> & mut Histogram < u64 > { self . maybe_rotate () ; & mut self . read } pub fn write (& mut self) -> & mut Histogram < u64 > { self . maybe_rotate () ; & mut self . write } fn maybe_rotate (& mut self) { let delta = Instant :: now () . saturating_duration_since (self . last_rotation) ; let rotations = (nanos (delta) / nanos (self . period)) as u32 ; if rotations >= 2 { trace ! ("Time since last rotation is {:?}.  clearing!" , delta) ; self . clear () ; } else if rotations == 1 { trace ! ("Time since last rotation is {:?}. rotating!" , delta) ; self . rotate () ; } self . last_rotation += self . period * rotations ; } fn rotate (& mut self) { std :: mem :: swap (& mut self . read , & mut self . write) ; trace ! ("Rotated {:?} points into read" , self . read . len ()) ; self . write . clear () ; } fn clear (& mut self) { self . read . clear () ; self . write . clear () ; } }
};
}
