// Generated macro for impl_478 (impl)
macro_rules! Depcrate_zerovec_databakeimpl_478 {
() => {
// Module: crate::zerovec::databake
// Provides: {"impl_478"}
// Dependencies: {}
impl < T : AsULE > Bake for ZeroVec < '_ , T > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; if self . is_empty () { quote ! { zerovec :: ZeroVec :: new () } } else { let bytes = databake :: Bake :: bake (& self . as_bytes () , env) ; quote ! { unsafe { zerovec :: ZeroVec :: from_bytes_unchecked (# bytes) } } } } }
};
}
