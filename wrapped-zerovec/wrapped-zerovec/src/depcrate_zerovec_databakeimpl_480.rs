// Generated macro for impl_480 (impl)
macro_rules! Depcrate_zerovec_databakeimpl_480 {
() => {
// Module: crate::zerovec::databake
// Provides: {"impl_480"}
// Dependencies: {}
impl < T : AsULE > Bake for & ZeroSlice < T > { fn bake (& self , env : & CrateEnv) -> TokenStream { env . insert ("zerovec") ; if self . is_empty () { quote ! { zerovec :: ZeroSlice :: new_empty () } } else { let bytes = databake :: Bake :: bake (& self . as_bytes () , env) ; quote ! { unsafe { zerovec :: ZeroSlice :: from_bytes_unchecked (# bytes) } } } } }
};
}
