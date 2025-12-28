macro_rules! deps {
    () => {
        InferConst!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < CTX > HashStable < CTX > for InferConst { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { match self { InferConst :: Var (_) => { panic ! ("const variables should not be hashed: {self:?}") } InferConst :: Fresh (i) => i . hash_stable (hcx , hasher) , } } }
    };
}

impl_285!();