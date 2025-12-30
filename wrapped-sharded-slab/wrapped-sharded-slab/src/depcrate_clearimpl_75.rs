// Generated macro for impl_75 (impl)
macro_rules! Depcrate_clearimpl_75 {
() => {
// Module: crate::clear
// Provides: {"impl_75"}
// Dependencies: {}
impl < T : Clear > Clear for sync :: Mutex < T > { # [inline] fn clear (& mut self) { self . get_mut () . unwrap () . clear () ; } }
};
}
