// Generated macro for impl_80 (impl)
macro_rules! Depcrate_map_borrowedimpl_80 {
() => {
// Module: crate::map::borrowed
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a , K , V > ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K : ? Sized , V : ? Sized , { # [doc (hidden)] pub const unsafe fn from_parts_unchecked (keys : & 'a < K as ZeroMapKV < 'a > > :: Slice , values : & 'a < V as ZeroMapKV < 'a > > :: Slice ,) -> Self { Self { keys , values } } # [doc = " The number of elements in the [`ZeroMapBorrowed`]"] pub fn len (self) -> usize { self . values . zvl_len () } # [doc = " Whether the [`ZeroMapBorrowed`] is empty"] pub fn is_empty (self) -> bool { self . values . zvl_len () == 0 } }
};
}
