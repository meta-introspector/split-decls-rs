// Generated macro for impl_126 (impl)
macro_rules! Depcrate_inherentimpl_126 {
() => {
// Module: crate::inherent
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a , T : Copy , const N : usize > SliceLike for & 'a [T ; N] { type Item = T ; type IntoIter = std :: iter :: Copied < std :: slice :: Iter < 'a , T > > ; fn iter (self) -> Self :: IntoIter { self . into_iter () . copied () } fn as_slice (& self) -> & [Self :: Item] { * self } }
};
}
