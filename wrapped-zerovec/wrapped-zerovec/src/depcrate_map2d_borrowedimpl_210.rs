// Generated macro for impl_210 (impl)
macro_rules! Depcrate_map2d_borrowedimpl_210 {
() => {
// Module: crate::map2d::borrowed
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > , K1 : ZeroMapKV < 'a > , V : ZeroMapKV < 'a > , K0 : ? Sized , K1 : ? Sized , V : ? Sized , { # [doc = " Produce an ordered iterator over keys0"] pub fn iter0 < 'l > (& 'l self) -> impl Iterator < Item = ZeroMap2dCursor < 'a , 'a , K0 , K1 , V > > + 'l { (0 .. self . keys0 . zvl_len ()) . map (move | idx | ZeroMap2dCursor :: from_borrowed (self , idx)) } }
};
}
