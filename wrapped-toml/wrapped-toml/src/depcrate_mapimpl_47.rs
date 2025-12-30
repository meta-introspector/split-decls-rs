// Generated macro for impl_47 (impl)
macro_rules! Depcrate_mapimpl_47 {
() => {
// Module: crate::map
// Provides: {"impl_47"}
// Dependencies: {}
impl < K , V > IntoIterator for Map < K , V > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IntoIter { iter : self . map . into_iter () , } } }
};
}
