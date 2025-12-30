// Generated macro for impl_83 (impl)
macro_rules! Depcrate_map_borrowedimpl_83 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a , K , V > ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + Ord + ? Sized , V : ZeroMapKV < 'a , Slice = ZeroSlice < V > > + AsULE + Copy + 'static , { # [doc = " For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`"] pub fn get_copied (self , key : & K) -> Option < V > { let index = self . keys . zvl_binary_search (key) . ok () ? ; self . values . get (index) } # [doc = " For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`"] pub fn get_copied_by (self , predicate : impl FnMut (& K) -> Ordering) -> Option < V > { let index = self . keys . zvl_binary_search_by (predicate) . ok () ? ; self . values . get (index) } # [doc = " Similar to [`Self::iter()`] except it returns a direct copy of the values instead of references"] # [doc = " to `V::ULE`, in cases when `V` is fixed-size"] pub fn iter_copied_values (self ,) -> impl Iterator < Item = (& 'a < K as ZeroMapKV < 'a > > :: GetType , V) > { (0 .. self . keys . zvl_len ()) . map (move | idx | { (# [expect (clippy :: unwrap_used)] self . keys . zvl_get (idx) . unwrap () , # [expect (clippy :: unwrap_used)] self . values . get (idx) . unwrap () ,) }) } }
};
}
