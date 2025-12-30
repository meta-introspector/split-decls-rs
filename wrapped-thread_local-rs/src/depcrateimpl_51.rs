// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , T : Send > IntoIterator for & 'a mut ThreadLocal < T > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
