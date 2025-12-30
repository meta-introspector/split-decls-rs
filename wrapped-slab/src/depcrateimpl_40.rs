// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a mut Slab < T > { type Item = (usize , & 'a mut T) ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
