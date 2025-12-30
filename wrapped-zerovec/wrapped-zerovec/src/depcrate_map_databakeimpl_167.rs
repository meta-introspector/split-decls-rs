// Generated macro for impl_167 (impl)
macro_rules! Depcrate_map_databakeimpl_167 {
() => {
// Module: crate::map::databake
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'a , K , V > Bake for ZeroMap < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , K :: Container : Bake , V :: Container : Bake , { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; let keys = self . keys . bake (env) ; let values = self . values . bake (env) ; quote ! { unsafe { # [allow (unused_unsafe)] zerovec :: ZeroMap :: from_parts_unchecked (# keys , # values) } } } }
};
}
