// Generated macro for impl_211 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_211 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + Ord , K1 : ZeroMapKV < 'a > + Ord , V : ZeroMapKV < 'a > , V : Copy , K0 : ? Sized , K1 : ? Sized , { # [doc = " For cases when `V` is fixed-size, obtain a direct copy of `V` instead of `V::ULE`"] pub fn get_copied_2d (& self , key0 : & K0 , key1 : & K1) -> Option < V > { self . get0 (key0) ? . get1_copied (key1) } }
};
}
