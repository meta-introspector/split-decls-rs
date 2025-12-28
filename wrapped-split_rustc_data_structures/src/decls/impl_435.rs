macro_rules! deps {
    () => {
        HashStable!();
        StableOrd!();
        SortedMap!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < K : HashStable < CTX > + StableOrd , V : HashStable < CTX > , CTX > HashStable < CTX > for SortedMap < K , V > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . data . hash_stable (ctx , hasher) ; } }
    };
}

impl_435!()