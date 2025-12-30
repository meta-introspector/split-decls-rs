// Generated macro for impl_363 (impl)
macro_rules! Depcrate_binderimpl_363 {
() => {
// Module: crate::binder
// Provides: {"impl_363"}
// Dependencies: {}
impl < I : Interner , T : Iterator > Iterator for EarlyBinderIter < I , T > { type Item = EarlyBinder < I , T :: Item > ; fn next (& mut self) -> Option < Self :: Item > { self . t . next () . map (| value | EarlyBinder { value , _tcx : PhantomData }) } fn size_hint (& self) -> (usize , Option < usize >) { self . t . size_hint () } }
};
}
