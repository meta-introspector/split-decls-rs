// Generated macro for impl_76 (impl)
macro_rules! Depcrate_clearimpl_76 {
() => {
// Module: crate::clear
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : Clear > Clear for sync :: RwLock < T > { # [inline] fn clear (& mut self) { self . write () . unwrap () . clear () ; } }
};
}
