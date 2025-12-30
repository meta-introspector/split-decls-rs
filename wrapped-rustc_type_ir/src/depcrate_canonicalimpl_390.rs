// Generated macro for impl_390 (impl)
macro_rules! Depcrate_canonicalimpl_390 {
() => {
// Module: crate::canonical
// Provides: {"impl_390"}
// Dependencies: {}
impl < 'a , I : Interner > IntoIterator for & 'a CanonicalVarValues < I > { type Item = I :: GenericArg ; type IntoIter = < I :: GenericArgs as SliceLike > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { self . var_values . iter () } }
};
}
