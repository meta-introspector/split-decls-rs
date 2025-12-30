// Generated macro for impl_505 (impl)
macro_rules! Depcrate_instantimpl_505 {
() => {
// Module: crate::instant
// Provides: {"impl_505"}
// Dependencies: {}
impl Add < Duration > for StdInstant { type Output = Self ; # [inline] fn add (self , duration : Duration) -> Self :: Output { (Instant (self) + duration) . 0 } }
};
}
