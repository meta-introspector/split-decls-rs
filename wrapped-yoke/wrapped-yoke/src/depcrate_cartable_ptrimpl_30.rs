// Generated macro for impl_30 (impl)
macro_rules! Depcrate_cartable_ptrimpl_30 {
() => {
// Module: crate::cartable_ptr
// Provides: {"impl_30"}
// Dependencies: {}
impl < C > Drop for CartableOptionPointer < C > where C : CartablePointerLike , { # [inline] fn drop (& mut self) { let ptr = self . inner ; if ptr != sentinel_for :: < C :: Raw > () { self . inner = sentinel_for :: < C :: Raw > () ; unsafe { C :: drop_raw (ptr) } } } }
};
}
