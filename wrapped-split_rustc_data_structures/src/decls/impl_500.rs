macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_500 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for f32 { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let val : u32 = self . to_bits () ; val . hash_stable (ctx , hasher) ; } }
    };
}

impl_500!()