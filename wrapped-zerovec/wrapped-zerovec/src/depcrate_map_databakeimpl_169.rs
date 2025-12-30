// Generated macro for impl_169 (impl)
macro_rules! Depcrate_map_databakeimpl_169 {
() => {
// Module: crate::map::databake
// Provides: {"impl_169"}
// Dependencies: {}
impl < 'a , K , V > Bake for ZeroMapBorrowed < 'a , K , V > where K : ZeroMapKV < 'a > + ? Sized , V : ZeroMapKV < 'a > + ? Sized , & 'a K :: Slice : Bake , & 'a V :: Slice : Bake , { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; let keys = self . keys . bake (env) ; let values = self . values . bake (env) ; quote ! { unsafe { # [allow (unused_unsafe)] zerovec :: maps :: ZeroMapBorrowed :: from_parts_unchecked (# keys , # values) } } } }
};
}
