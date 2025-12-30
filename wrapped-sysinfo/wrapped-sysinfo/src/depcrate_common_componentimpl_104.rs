// Generated macro for impl_104 (impl)
macro_rules! Depcrate_common_componentimpl_104 {
() => {
// Module: crate::common::component
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Components { type Item = & 'a mut Component ; type IntoIter = std :: slice :: IterMut < 'a , Component > ; fn into_iter (self) -> Self :: IntoIter { self . list_mut () . iter_mut () } }
};
}
