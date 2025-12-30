// Generated macro for impl_24 (impl)
macro_rules! Depcrate_mapimpl_24 {
() => {
// Module: crate::map
// Provides: {"impl_24"}
// Dependencies: {}
impl < K : Ord + Hash , V > FromIterator < (K , V) > for Map < K , V > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = (K , V) > , { Self { map : FromIterator :: from_iter (iter) , dotted : false , implicit : false , inline : false , } } }
};
}
