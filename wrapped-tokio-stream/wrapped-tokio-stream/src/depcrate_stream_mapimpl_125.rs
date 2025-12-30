// Generated macro for impl_125 (impl)
macro_rules! Depcrate_stream_mapimpl_125 {
() => {
// Module: crate::stream_map
// Provides: {"impl_125"}
// Dependencies: {}
impl < K , V > Extend < (K , V) > for StreamMap < K , V > { fn extend < T > (& mut self , iter : T) where T : IntoIterator < Item = (K , V) > , { self . entries . extend (iter) ; } }
};
}
