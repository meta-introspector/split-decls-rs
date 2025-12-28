macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for f64 { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let val : u64 = self . to_bits () ; val . hash_stable (ctx , hasher) ; } }
    };
}

impl_501!()