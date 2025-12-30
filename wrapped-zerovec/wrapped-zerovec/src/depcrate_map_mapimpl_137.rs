// Generated macro for impl_137 (impl)
macro_rules! Depcrate_map_mapimpl_137 {
() => {
// Module: crate::map::map
// Provides: {"impl_137"}
// Dependencies: {}
impl < 'a , K , V > ZeroMap < 'a , K , V > where K : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , K > > , V : AsULE + ZeroMapKV < 'a , Container = ZeroVec < 'a , V > > , { # [doc = " Similar to [`Self::iter()`] except it returns a direct copy of the keys values instead of references"] # [doc = " to `K::ULE` and `V::ULE`, in cases when `K` and `V` are fixed-size"] pub fn iter_copied < 'b > (& 'b self) -> impl Iterator < Item = (K , V) > + 'b { let keys = & self . keys ; let values = & self . values ; (0 .. keys . len ()) . map (move | idx | { (# [expect (clippy :: unwrap_used)] ZeroSlice :: get (& * * keys , idx) . unwrap () , # [expect (clippy :: unwrap_used)] ZeroSlice :: get (& * * values , idx) . unwrap () ,) }) } }
};
}
