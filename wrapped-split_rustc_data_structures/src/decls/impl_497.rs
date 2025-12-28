macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_497 {
    () => {
        deps!();
        impl < CTX , T > HashStable < CTX > for PhantomData < T > { fn hash_stable (& self , _ctx : & mut CTX , _hasher : & mut StableHasher) { } }
    };
}

impl_497!()