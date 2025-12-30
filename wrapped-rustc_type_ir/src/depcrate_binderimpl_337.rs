// Generated macro for impl_337 (impl)
macro_rules! Depcrate_binderimpl_337 {
() => {
// Module: crate::binder
// Provides: {"impl_337"}
// Dependencies: {}
impl < I : Interner , T : IntoIterator > Binder < I , T > { pub fn iter (self) -> impl Iterator < Item = Binder < I , T :: Item > > { let Binder { value , bound_vars } = self ; value . into_iter () . map (move | value | Binder { value , bound_vars }) } }
};
}
