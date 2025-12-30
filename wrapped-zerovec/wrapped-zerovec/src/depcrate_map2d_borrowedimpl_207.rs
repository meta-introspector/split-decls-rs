// Generated macro for impl_207 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_207 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_207"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { # [doc (hidden)] pub const unsafe fn from_parts_unchecked (keys0 : & 'a K0 :: Slice , joiner : & 'a ZeroSlice < u32 > , keys1 : & 'a K1 :: Slice , values : & 'a V :: Slice ,) -> Self { Self { keys0 , joiner , keys1 , values , } } # [doc = " The number of elements in the [`ZeroMap2dBorrowed`]"] pub fn len (& self) -> usize { self . values . zvl_len () } # [doc = " Whether the [`ZeroMap2dBorrowed`] is empty"] pub fn is_empty (& self) -> bool { self . values . zvl_len () == 0 } }
};
}
