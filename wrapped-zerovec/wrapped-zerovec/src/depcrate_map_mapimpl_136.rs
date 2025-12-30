// Generated macro for impl_136 (impl)
macro_rules! Depcrate_map_mapimpl_136 {
() => {
// Module: crate::map::map
// Provides: {"impl_136"}
// Dependencies: {}
impl < 'a , K , V > ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , V > > , { # [doc = " Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references"] # [doc = " to `V::ULE`, in cases when `V` is fixed-size"] pub fn iter_copied_values < 'b > (& 'b self ,) -> impl Iterator < Item = (& 'b < K as ZeroMapKV < 'a > > :: GetType , V) > { (0 .. self . keys . zvl_len ()) . map (move | idx | { (# [expect (clippy :: unwrap_used)] self . keys . zvl_get (idx) . unwrap () , # [expect (clippy :: unwrap_used)] ZeroSlice :: get (& * self . values , idx) . unwrap () ,) }) } }
};
}
