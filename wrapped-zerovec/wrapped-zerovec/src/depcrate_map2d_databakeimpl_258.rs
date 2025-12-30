// Generated macro for impl_258 (impl)
macro_rules! Depcrate_map2d_databakeimpl_258 {
() => {
// Module: crate::map2d::databake
// Provides: {"impl_258"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > BakeSize for ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K0 :: Container : BakeSize , K1 :: Container : BakeSize , V :: Container : BakeSize , { fn borrows_size (& self) -> usize { self . keys0 . borrows_size () + self . joiner . borrows_size () + self . keys1 . borrows_size () + self . values . borrows_size () } }
};
}
