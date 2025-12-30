// Generated macro for impl_439 (impl)
macro_rules! Depcrate_varzerovec_databakeimpl_439 {
() => {
// Module: crate::varzerovec::databake
// Provides: {"impl_439"}
// Dependencies: {}
impl < T : VarULE + ? Sized > Bake for & VarZeroSlice < T , Index16 > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; if self . is_empty () { quote ! { zerovec :: vecs :: VarZeroSlice16 :: new_empty () } } else { let bytes = databake :: Bake :: bake (& self . as_bytes () , env) ; quote ! { unsafe { zerovec :: vecs :: VarZeroSlice16 :: from_bytes_unchecked (# bytes) } } } } }
};
}
