// Generated macro for impl_598 (impl)
macro_rules! Depcrate_sso_mapimpl_598 {
() => {
// Module: crate::sso::map
// Provides: {"impl_598"}
// Dependencies: {}
impl < 'a , K , V > IntoIterator for & 'a SsoHashMap < K , V > { type IntoIter = Either < std :: iter :: Map < < & 'a ArrayVec < (K , V) , SSO_ARRAY_SIZE > as IntoIterator > :: IntoIter , fn (& 'a (K , V)) -> (& 'a K , & 'a V) , > , < & 'a FxHashMap < K , V > as IntoIterator > :: IntoIter , > ; type Item = < Self :: IntoIter as Iterator > :: Item ; fn into_iter (self) -> Self :: IntoIter { match self { SsoHashMap :: Array (array) => Either :: Left (array . into_iter () . map (adapt_array_ref_it)) , SsoHashMap :: Map (map) => Either :: Right (map . iter ()) , } } }
};
}
