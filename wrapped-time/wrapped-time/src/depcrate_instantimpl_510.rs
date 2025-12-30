// Generated macro for impl_510 (impl)
macro_rules! Depcrate_instantimpl_510 {
() => {
// Module: crate::instant
// Provides: {"impl_510"}
// Dependencies: {}
impl Sub < Duration > for StdInstant { type Output = Self ; # [inline] fn sub (self , duration : Duration) -> Self :: Output { (Instant (self) - duration) . 0 } }
};
}
