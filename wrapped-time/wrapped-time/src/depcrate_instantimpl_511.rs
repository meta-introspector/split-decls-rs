// Generated macro for impl_511 (impl)
macro_rules! Depcrate_instantimpl_511 {
() => {
// Module: crate::instant
// Provides: {"impl_511"}
// Dependencies: {}
impl Sub < StdDuration > for Instant { type Output = Self ; # [doc = " # Panics"] # [doc = ""] # [doc = " This function may panic if the resulting point in time cannot be represented by the"] # [doc = " underlying data structure."] # [inline] fn sub (self , duration : StdDuration) -> Self :: Output { # [expect (clippy :: unchecked_duration_subtraction)] Self (self . 0 - duration) } }
};
}
