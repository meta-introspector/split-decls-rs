// Generated macro for impl_871 (impl)
macro_rules! Depcrate_punctuatedimpl_871 {
() => {
// Module: crate::punctuated
// Provides: {"impl_871"}
// Dependencies: {}
impl < 'a , T , P > IntoIterator for & 'a mut Punctuated < T , P > { type Item = & 'a mut T ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { Punctuated :: iter_mut (self) } }
};
}
