// Generated macro for impl_70 (impl)
macro_rules! Depcrate_clearimpl_70 {
() => {
// Module: crate::clear
// Provides: {"impl_70"}
// Dependencies: {}
impl < T > Clear for Box < T > where T : Clear , { # [inline] fn clear (& mut self) { self . deref_mut () . clear () } }
};
}
