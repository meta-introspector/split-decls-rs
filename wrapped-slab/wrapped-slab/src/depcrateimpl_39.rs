// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a Slab < T > { type Item = (usize , & 'a T) ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
