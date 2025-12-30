// Generated macro for impl_44 (impl)
macro_rules! Depcrate_collectorimpl_44 {
() => {
// Module: crate::collector
// Provides: {"impl_44"}
// Dependencies: {}
impl < 'tcx > Extend < Spanned < MonoItem < 'tcx > > > for MonoItems < 'tcx > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Spanned < MonoItem < 'tcx > > > , { for item in iter { self . push (item) } } }
};
}
