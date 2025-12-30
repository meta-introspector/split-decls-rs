// Generated macro for impl_221 (impl)
macro_rules! Depcrate_common_userimpl_221 {
() => {
// Module: crate::common::user
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Users { type Item = & 'a User ; type IntoIter = std :: slice :: Iter < 'a , User > ; fn into_iter (self) -> Self :: IntoIter { self . list () . iter () } }
};
}
