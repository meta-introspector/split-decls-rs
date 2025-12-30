// Generated macro for impl_64 (impl)
macro_rules! Depcrate_hashmapimpl_64 {
() => {
// Module: crate::hashmap
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , K , V > ZeroHashMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , { pub fn iter < 'b > (& 'b self ,) -> impl ExactSizeIterator < Item = (& 'b < K as ZeroMapKV < 'a > > :: GetType , & 'b < V as ZeroMapKV < 'a > > :: GetType ,) , > { (0 .. self . len ()) . map (| index | { (# [expect (clippy :: unwrap_used)] self . keys . zvl_get (index) . unwrap () , # [expect (clippy :: unwrap_used)] self . values . zvl_get (index) . unwrap () ,) }) } pub fn iter_keys < 'b > (& 'b self ,) -> impl ExactSizeIterator < Item = & 'b < K as ZeroMapKV < 'a > > :: GetType > { # [expect (clippy :: unwrap_used)] (0 .. self . len ()) . map (| index | self . keys . zvl_get (index) . unwrap ()) } pub fn iter_values < 'b > (& 'b self ,) -> impl ExactSizeIterator < Item = & 'b < V as ZeroMapKV < 'a > > :: GetType > { # [expect (clippy :: unwrap_used)] (0 .. self . len ()) . map (| index | self . values . zvl_get (index) . unwrap ()) } }
};
}
