// Generated macro for impl_336 (impl)
macro_rules! Depcrate_binderimpl_336 {
() => {
// Module: crate::binder
// Provides: {"impl_336"}
// Dependencies: {}
impl < I : Interner , T > Binder < I , Option < T > > { pub fn transpose (self) -> Option < Binder < I , T > > { let Binder { value , bound_vars } = self ; value . map (| value | Binder { value , bound_vars }) } }
};
}
