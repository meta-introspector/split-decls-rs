// Generated macro for impl_37 (impl)
macro_rules! Depcrate_mapimpl_37 {
() => {
// Module: crate::map
// Provides: {"impl_37"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a Map < K , V > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { Iter { iter : self . map . iter () , } } }
};
}
