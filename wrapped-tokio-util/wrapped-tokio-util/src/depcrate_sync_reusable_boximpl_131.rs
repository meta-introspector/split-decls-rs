// Generated macro for impl_131 (impl)
macro_rules! Depcrate_sync_reusable_boximpl_131 {
() => {
// Module: crate::sync::reusable_box
// Provides: {"impl_131"}
// Dependencies: {}
impl < O , F : FnOnce () -> O > CallOnDrop < O , F > { fn new (f : F) -> Self { let f = ManuallyDrop :: new (f) ; Self { f } } fn call (self) -> O { let mut this = ManuallyDrop :: new (self) ; let f = unsafe { ManuallyDrop :: take (& mut this . f) } ; f () } }
};
}
