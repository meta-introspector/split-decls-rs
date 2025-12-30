// Generated macro for impl_874 (impl)
macro_rules! Depcrate_unordimpl_874 {
() => {
// Module: crate::unord
// Provides: {"impl_874"}
// Dependencies: {}
impl < K : Hash + Eq , V , I : Iterator < Item = (K , V) > > From < UnordItems < (K , V) , I > > for UnordMap < K , V > { # [inline] fn from (items : UnordItems < (K , V) , I >) -> Self { UnordMap { inner : FxHashMap :: from_iter (items . 0) } } }
};
}
