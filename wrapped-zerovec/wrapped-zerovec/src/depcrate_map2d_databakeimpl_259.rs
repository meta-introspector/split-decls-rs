// Generated macro for impl_259 (impl)
macro_rules! Depcrate_map2d_databakeimpl_259 {
() => {
// Module: crate::map2d::databake
// Provides: {"impl_259"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > Bake for ZeroMap2dBorrowed < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , & 'a K0 :: Slice : Bake , & 'a K1 :: Slice : Bake , & 'a V :: Slice : Bake , { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; let keys0 = self . keys0 . bake (env) ; let joiner = self . joiner . bake (env) ; let keys1 = self . keys1 . bake (env) ; let values = self . values . bake (env) ; quote ! { unsafe { # [allow (unused_unsafe)] zerovec :: maps :: ZeroMap2dBorrowed :: from_parts_unchecked (# keys0 , # joiner , # keys1 , # values) } } } }
};
}
