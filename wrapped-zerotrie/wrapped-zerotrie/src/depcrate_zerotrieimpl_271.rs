// Generated macro for impl_271 (impl)
macro_rules! Depcrate_zerotrieimpl_271 {
() => {
// Module: crate::zerotrie
// Provides: {"impl_271"}
// Dependencies: {}
# [cfg (feature = "databake")] impl < Store > databake :: Bake for ZeroTrie < Store > where Store : databake :: Bake , { fn bake (& self , env : & databake :: CrateEnv) -> databake :: TokenStream { use databake :: * ; let inner = impl_dispatch ! (& self , bake (env)) ; quote ! { # inner . into_zerotrie () } } }
};
}
