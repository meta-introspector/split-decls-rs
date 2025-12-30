// Generated macro for impl_56 (impl)
macro_rules! Depcrate_bagimpl_56 {
() => {
// Module: crate::bag
// Provides: {"impl_56"}
// Dependencies: {}
impl < T , const ARRAY_LEN : usize > IntoIterator for Bag < T , ARRAY_LEN > { type Item = T ; type IntoIter = IntoIter < T , ARRAY_LEN > ; # [inline] fn into_iter (self) -> Self :: IntoIter { IntoIter { bag : self } } }
};
}
