// Generated macro for impl_362 (impl)
macro_rules! Depcrate_binderimpl_362 {
() => {
// Module: crate::binder
// Provides: {"impl_362"}
// Dependencies: {}
impl < I : Interner , T : IntoIterator > EarlyBinder < I , T > { pub fn transpose_iter (self) -> EarlyBinderIter < I , T :: IntoIter > { EarlyBinderIter { t : self . value . into_iter () , _tcx : PhantomData } } }
};
}
