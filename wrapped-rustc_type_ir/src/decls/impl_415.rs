macro_rules! deps {
    () => {
        RegionKind!();
        Interner!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < CTX , I : Interner > HashStable < CTX > for RegionKind < I > where I :: EarlyParamRegion : HashStable < CTX > , I :: BoundRegion : HashStable < CTX > , I :: LateParamRegion : HashStable < CTX > , I :: PlaceholderRegion : HashStable < CTX > , { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { std :: mem :: discriminant (self) . hash_stable (hcx , hasher) ; match self { ReErased | ReStatic | ReError (_) => { } ReBound (d , r) => { d . hash_stable (hcx , hasher) ; r . hash_stable (hcx , hasher) ; } ReEarlyParam (r) => { r . hash_stable (hcx , hasher) ; } ReLateParam (r) => { r . hash_stable (hcx , hasher) ; } RePlaceholder (r) => { r . hash_stable (hcx , hasher) ; } ReVar (_) => { panic ! ("region variables should not be hashed: {self:?}") } } } }
    };
}

impl_415!()