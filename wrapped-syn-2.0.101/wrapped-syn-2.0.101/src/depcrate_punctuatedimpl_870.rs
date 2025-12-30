// Generated macro for impl_870 (impl)
macro_rules! Depcrate_punctuatedimpl_870 {
() => {
// Module: crate::punctuated
// Provides: {"impl_870"}
// Dependencies: {}
impl < 'a , T , P > IntoIterator for & 'a Punctuated < T , P > { type Item = & 'a T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Self :: IntoIter { Punctuated :: iter (self) } }
};
}
