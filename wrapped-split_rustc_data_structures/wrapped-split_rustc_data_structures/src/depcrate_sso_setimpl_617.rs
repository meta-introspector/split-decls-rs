// Generated macro for impl_617 (impl)
macro_rules! Depcrate_sso_setimpl_617 {
() => {
// Module: crate::sso::set
// Provides: {"impl_617"}
// Dependencies: {}
impl < T > IntoIterator for SsoHashSet < T > { type IntoIter = std :: iter :: Map < < SsoHashMap < T , () > as IntoIterator > :: IntoIter , fn ((T , ())) -> T > ; type Item = < Self :: IntoIter as Iterator > :: Item ; # [inline] fn into_iter (self) -> Self :: IntoIter { self . map . into_iter () . map (entry_to_key) } }
};
}
