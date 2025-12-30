// Generated macro for impl_231 (impl)
macro_rules! Depcrate_common_userimpl_231 {
() => {
// Module: crate::common::user
// Provides: {"impl_231"}
// Dependencies: {}
impl < 'a > IntoIterator for & 'a mut Groups { type Item = & 'a mut Group ; type IntoIter = std :: slice :: IterMut < 'a , Group > ; fn into_iter (self) -> Self :: IntoIter { self . list_mut () . iter_mut () } }
};
}
