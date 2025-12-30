// Generated macro for impl_346 (impl)
macro_rules! Depcrate_binderimpl_346 {
() => {
// Module: crate::binder
// Provides: {"impl_346"}
// Dependencies: {}
impl < I : Interner , T > EarlyBinder < I , Option < T > > { pub fn transpose (self) -> Option < EarlyBinder < I , T > > { self . value . map (| value | EarlyBinder { value , _tcx : PhantomData }) } }
};
}
