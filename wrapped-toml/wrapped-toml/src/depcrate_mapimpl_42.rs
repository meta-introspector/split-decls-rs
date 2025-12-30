// Generated macro for impl_42 (impl)
macro_rules! Depcrate_mapimpl_42 {
() => {
// Module: crate::map
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a mut Map < K , V > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IterMut { iter : self . map . iter_mut () , } } }
};
}
