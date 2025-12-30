// Generated macro for impl_435 (impl)
macro_rules! Depcrate_varzerovec_databakeimpl_435 {
() => {
// Module: crate::varzerovec::databake
// Provides: {"impl_435"}
// Dependencies: {}
impl < T : VarULE + ? Sized > Bake for VarZeroVec < '_ , T , Index16 > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; if self . is_empty () { quote ! { zerovec :: vecs :: VarZeroVec16 :: new () } } else { let bytes = databake :: Bake :: bake (& self . as_bytes () , env) ; quote ! { unsafe { zerovec :: vecs :: VarZeroVec16 :: from_bytes_unchecked (# bytes) } } } } }
};
}
