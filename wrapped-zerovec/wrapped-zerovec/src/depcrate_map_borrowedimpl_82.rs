// Generated macro for impl_82 (impl)
macro_rules! Depcrate_map_borrowedimpl_82 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'a , K , V > ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , { # [doc = " Produce an ordered iterator over key-value pairs"] pub fn iter (self ,) -> impl Iterator < Item = (& 'a < K as ZeroMapKV < 'a > > :: GetType , & 'a < V as ZeroMapKV < 'a > > :: GetType ,) , > { self . iter_keys () . zip (self . iter_values ()) } # [doc = " Produce an ordered iterator over keys"] pub fn iter_keys (self) -> impl Iterator < Item = & 'a < K as ZeroMapKV < 'a > > :: GetType > { # [expect (clippy :: unwrap_used)] (0 .. self . keys . zvl_len ()) . map (move | idx | self . keys . zvl_get (idx) . unwrap ()) } # [doc = " Produce an iterator over values, ordered by keys"] pub fn iter_values (self) -> impl Iterator < Item = & 'a < V as ZeroMapKV < 'a > > :: GetType > { # [expect (clippy :: unwrap_used)] (0 .. self . values . zvl_len ()) . map (move | idx | self . values . zvl_get (idx) . unwrap ()) } }
};
}
