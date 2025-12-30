// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
impl ErrorImpl < Erased > { pub (crate) fn error (& self) -> & (dyn Error + Send + Sync + 'static) { unsafe { (self . vtable . object_ref) (self) } } }
};
}
