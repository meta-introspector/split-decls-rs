// Generated macro for impl_232 (impl)
macro_rules! Depcrate_dropsimpl_232 {
() => {
// Module: crate::drops
// Provides: {"impl_232"}
// Dependencies: {}
impl < T > NoDrop < T > { pub (crate) fn new (value : T) -> Self where T : TrivialDrop , { NoDrop (ManuallyDrop :: new (value)) } }
};
}
