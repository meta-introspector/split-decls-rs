// Generated macro for impl_257 (impl)
macro_rules! Depcrate_map2d_databakeimpl_257 {
() => {
// Module: crate::map2d::databake
// Provides: {"impl_257"}
// Dependencies: {}
impl < 'a , K0 , K1 , V > Bake for ZeroMap2d < 'a , K0 , K1 , V > where K0 : ZeroMapKV < 'a > + ? Sized , K1 : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K0 :: Container : Bake , K1 :: Container : Bake , V :: Container : Bake , { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; let keys0 = self . keys0 . bake (env) ; let joiner = self . joiner . bake (env) ; let keys1 = self . keys1 . bake (env) ; let values = self . values . bake (env) ; quote ! { unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap2d :: from_parts_unchecked (# keys0 , # joiner , # keys1 , # values) } } } }
};
}
