// Generated macro for impl_103 (impl)
macro_rules! Depcrate_common_componentimpl_103 {
() => {
// Module: crate::common::component
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a Components { type Item = & 'a Component ; type IntoIter = std :: slice :: Iter < 'a , Component > ; fn into_iter (self) -> Self :: IntoIter { self . list () . iter () } }
};
}
