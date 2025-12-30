// Generated macro for impl_32 (impl)
macro_rules! Depcrate_cowimpl_32 {
() => {
// Module: crate::cow
// Provides: {"impl_32"}
// Dependencies: {}
# [cfg (feature = "databake")] impl < 'a , V : VarULE + ? Sized > databake :: Bake for VarZeroCow < 'a , V > { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { env . insert ("zerovec") ; let bytes = self . as_bytes () . bake (env) ; databake :: quote ! { unsafe { zerovec :: VarZeroCow :: from_bytes_unchecked (# bytes) } } } }
};
}
