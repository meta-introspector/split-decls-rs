// Generated macro for impl_222 (impl)
macro_rules! Depcrate_common_userimpl_222 {
() => {
// Module: crate::common::user
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Users { type Item = & 'a mut User ; type IntoIter = std :: slice :: IterMut < 'a , User > ; fn into_iter (self) -> Self :: IntoIter { self . list_mut () . iter_mut () } }
};
}
