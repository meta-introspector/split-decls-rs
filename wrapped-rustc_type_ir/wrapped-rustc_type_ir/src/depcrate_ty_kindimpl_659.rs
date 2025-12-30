// Generated macro for impl_659 (impl)
macro_rules! Depcrate_ty_kindimpl_659 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_659"}
// Dependencies: {}
# [cfg (feature = "nightly")] impl < CTX > HashStable < CTX > for InferTy { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { use InferTy :: * ; std :: mem :: discriminant (self) . hash_stable (ctx , hasher) ; match self { TyVar (_) | IntVar (_) | FloatVar (_) => { panic ! ("type variables should not be hashed: {self:?}") } FreshTy (v) | FreshIntTy (v) | FreshFloatTy (v) => v . hash_stable (ctx , hasher) , } } }
};
}
