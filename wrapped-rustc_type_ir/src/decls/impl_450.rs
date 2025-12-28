macro_rules! deps {
    () => {
        InferTy!();
    };
}

macro_rules! impl_450 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < CTX > HashStable < CTX > for InferTy { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { use InferTy :: * ; std :: mem :: discriminant (self) . hash_stable (ctx , hasher) ; match self { TyVar (_) | IntVar (_) | FloatVar (_) => { panic ! ("type variables should not be hashed: {self:?}") } FreshTy (v) | FreshIntTy (v) | FreshFloatTy (v) => v . hash_stable (ctx , hasher) , } } }
    };
}

impl_450!()