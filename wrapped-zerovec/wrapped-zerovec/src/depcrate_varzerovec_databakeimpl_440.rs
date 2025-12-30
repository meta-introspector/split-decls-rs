// Generated macro for impl_440 (impl)
macro_rules! Depcrate_varzerovec_databakeimpl_440 {
() => {
// Module: crate::varzerovec::databake
// Provides: {"impl_440"}
// Dependencies: {}
impl < T : VarULE + ? Sized > Bake for & VarZeroSlice < T , Index32 > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; if self . is_empty () { quote ! { zerovec :: vecs :: VarZeroSlice32 :: new_empty () } } else { let bytes = databake :: Bake :: bake (& self . as_bytes () , env) ; quote ! { unsafe { zerovec :: vecs :: VarZeroSlice32 :: from_bytes_unchecked (# bytes) } } } } }
};
}
