// Generated macro for hash_stable_discriminant (function)
macro_rules! Depcrate_hash_stablehash_stable_discriminant {
() => {
// Module: crate::hash_stable
// Provides: {"hash_stable_discriminant"}
// Dependencies: {}
fn hash_stable_discriminant (s : & mut synstructure :: Structure < '_ >) -> proc_macro2 :: TokenStream { match s . ast () . data { syn :: Data :: Enum (_) => quote ! { :: std :: mem :: discriminant (self) . hash_stable (__hcx , __hasher) ; } , syn :: Data :: Struct (_) => quote ! { } , syn :: Data :: Union (_) => panic ! ("cannot derive on union") , } }
};
}
