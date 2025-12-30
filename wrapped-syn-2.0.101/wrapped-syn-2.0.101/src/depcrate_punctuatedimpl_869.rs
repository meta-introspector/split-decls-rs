// Generated macro for impl_869 (impl)
macro_rules! Depcrate_punctuatedimpl_869 {
() => {
// Module: crate::punctuated
// Provides: {"impl_869"}
// Dependencies: {}
impl < T , P > IntoIterator for Punctuated < T , P > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> Self :: IntoIter { let mut elements = Vec :: with_capacity (self . len ()) ; elements . extend (self . inner . into_iter () . map (| pair | pair . 0)) ; elements . extend (self . last . map (| t | * t)) ; IntoIter { inner : elements . into_iter () , } } }
};
}
