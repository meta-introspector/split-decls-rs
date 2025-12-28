macro_rules! deps {
    () => {
        HashStableContext!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl < H : HashStableContext > HashStable < H > for RelativeBytePos { fn hash_stable (& self , hcx : & mut H , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }
    };
}

impl_324!()