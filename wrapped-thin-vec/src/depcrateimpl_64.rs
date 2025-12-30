// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a mut ThinVec < T > { type Item = & 'a mut T ; type IntoIter = slice :: IterMut < 'a , T > ; fn into_iter (self) -> slice :: IterMut < 'a , T > { self . iter_mut () } }
};
}
