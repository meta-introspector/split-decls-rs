// Generated macro for impl_639 (impl)
macro_rules! Depcrate_ule_nicheimpl_639 {
() => {
// Module: crate::ule::niche
// Provides: {"impl_639"}
// Dependencies: {}
impl < T , const N : usize > IntoIterator for NichedOption < T , N > { type IntoIter = < Option < T > as IntoIterator > :: IntoIter ; type Item = T ; fn into_iter (self) -> Self :: IntoIter { self . 0 . into_iter () } }
};
}
