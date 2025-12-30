// Generated macro for impl_260 (impl)
macro_rules! Depcrate_map2d_databakeimpl_260 {
() => {
// Module: crate::map2d::databake
// Provides: {"impl_260"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > BakeSize for ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , & 'a K0 :: Slice : BakeSize , & 'a K1 :: Slice : BakeSize , & 'a V :: Slice : BakeSize , { fn borrows_size (& self) -> usize { self . keys0 . borrows_size () + self . joiner . borrows_size () + self . keys1 . borrows_size () + self . values . borrows_size () } }
};
}
