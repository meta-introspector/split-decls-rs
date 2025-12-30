// Generated macro for impl_131 (impl)
macro_rules! Depcrate_map_mapimpl_131 {
() => {
// Module: crate::map::map
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a , K , V > ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , { # [doc = " Produce an ordered iterator over key-value pairs"] pub fn iter < 'b > (& 'b self ,) -> impl ExactSizeIterator < Item = (& 'b < K as ZeroMapKV < 'a > > :: GetType , & 'b < V as ZeroMapKV < 'a > > :: GetType ,) , > { (0 .. self . keys . zvl_len ()) . map (move | idx | { (# [expect (clippy :: unwrap_used)] self . keys . zvl_get (idx) . unwrap () , # [expect (clippy :: unwrap_used)] self . values . zvl_get (idx) . unwrap () ,) }) } # [doc = " Produce an ordered iterator over keys"] pub fn iter_keys < 'b > (& 'b self ,) -> impl ExactSizeIterator < Item = & 'b < K as ZeroMapKV < 'a > > :: GetType > { # [expect (clippy :: unwrap_used)] (0 .. self . keys . zvl_len ()) . map (move | idx | self . keys . zvl_get (idx) . unwrap ()) } # [doc = " Produce an iterator over values, ordered by keys"] pub fn iter_values < 'b > (& 'b self ,) -> impl ExactSizeIterator < Item = & 'b < V as ZeroMapKV < 'a > > :: GetType > { # [expect (clippy :: unwrap_used)] (0 .. self . values . zvl_len ()) . map (move | idx | self . values . zvl_get (idx) . unwrap ()) } }
};
}
