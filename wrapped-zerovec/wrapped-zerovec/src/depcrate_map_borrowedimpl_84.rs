// Generated macro for impl_84 (impl)
macro_rules! Depcrate_map_borrowedimpl_84 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a , K , V > ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a , Slice = ZeroSlice < K > > + AsULE + Copy + Ord + 'static , V : ZeroMapKV < 'a , Slice = ZeroSlice < V > > + AsULE + Copy + 'static , { # [doc = " Similar to [`Self::iter()`] except it returns a direct copy of the keys values instead of references"] # [doc = " to `K::ULE` and `V::ULE`, in cases when `K` and `V` are fixed-size"] pub fn iter_copied (self) -> impl Iterator < Item = (K , V) > + 'a { let len = self . keys . zvl_len () ; (0 .. len) . map (move | idx | { (# [expect (clippy :: unwrap_used)] ZeroSlice :: get (self . keys , idx) . unwrap () , # [expect (clippy :: unwrap_used)] ZeroSlice :: get (self . values , idx) . unwrap () ,) }) } }
};
}
