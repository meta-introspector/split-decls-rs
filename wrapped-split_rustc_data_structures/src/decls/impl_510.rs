macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_510 {
    () => {
        deps!();
        impl < T : HashStable < CTX > , CTX > HashStable < CTX > for [T] { default fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for item in self { item . hash_stable (ctx , hasher) ; } } }
    };
}

impl_510!()