// Generated macro for impl_43 (impl)
macro_rules! Depcrate_collectorimpl_43 {
() => {
// Module: crate::collector
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'tcx > IntoIterator for MonoItems < 'tcx > { type Item = Spanned < MonoItem < 'tcx > > ; type IntoIter = impl Iterator < Item = Spanned < MonoItem < 'tcx > > > ; fn into_iter (self) -> Self :: IntoIter { self . items . into_iter () . map (| (item , span) | respan (span , item)) } }
};
}
