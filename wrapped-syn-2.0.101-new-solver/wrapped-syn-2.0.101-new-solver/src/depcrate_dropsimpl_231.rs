// Generated macro for impl_231 (impl)
macro_rules! Depcrate_dropsimpl_231 {
() => {
// Module: crate::drops
// Provides: {"impl_231"}
// Dependencies: {}
impl < T > NoDrop < T > { pub (crate) fn new (value : T) -> Self where T : TrivialDrop , { NoDrop (ManuallyDrop :: new (value)) } }
};
}
