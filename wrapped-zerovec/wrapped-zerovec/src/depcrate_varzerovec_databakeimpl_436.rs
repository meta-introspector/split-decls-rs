// Generated macro for impl_436 (impl)
macro_rules! Depcrate_varzerovec_databakeimpl_436 {
() => {
// Module: crate::varzerovec::databake
// Provides: {"impl_436"}
// Dependencies: {}
impl < T : VarULE + ? Sized > Bake for VarZeroVec < '_ , T , Index32 > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; if self . is_empty () { quote ! { zerovec :: vecs :: VarZeroVec32 :: new () } } else { let bytes = databake :: Bake :: bake (& self . as_bytes () , env) ; quote ! { unsafe { zerovec :: vecs :: VarZeroVec32 :: from_bytes_unchecked (# bytes) } } } } }
};
}
