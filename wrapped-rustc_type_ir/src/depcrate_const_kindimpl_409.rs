// Generated macro for impl_409 (impl)
macro_rules! Depcrate_const_kindimpl_409 {
() => {
// Module: crate::const_kind
// Provides: {"impl_409"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < CTX > HashStable < CTX > for InferConst { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { match self { InferConst :: Var (_) => { panic ! ("const variables should not be hashed: {self:?}") } InferConst :: Fresh (i) => i . hash_stable (hcx , hasher) , } } }
};
}
