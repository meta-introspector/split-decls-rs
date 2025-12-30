// Generated macro for impl_124 (impl)
macro_rules! Depcrate_stream_mapimpl_124 {
() => {
// Module: crate::stream_map
// Provides: {"impl_124"}
// Dependencies: {}
impl < K , V > FromIterator < (K , V) > for StreamMap < K , V > where K : Hash + Eq , { fn from_iter < T : IntoIterator < Item = (K , V) > > (iter : T) -> Self { let iterator = iter . into_iter () ; let (lower_bound , _) = iterator . size_hint () ; let mut stream_map = Self :: with_capacity (lower_bound) ; for (key , value) in iterator { stream_map . insert (key , value) ; } stream_map } }
};
}
