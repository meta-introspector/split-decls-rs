// Generated macro for impl_215 (impl)
macro_rules! Depcrate_collectibleimpl_215 {
() => {
// Module: crate::collectible
// Provides: {"impl_215"}
// Dependencies: {}
impl < F : 'static + FnOnce () > Drop for DeferredClosure < F > { # [inline] fn drop (& mut self) { if let Some (f) = self . f . take () { f () ; } } }
};
}
