macro_rules! deps {
    () => {
        ByteSymbol!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for ByteSymbol { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_byte_str () . hash_stable (hcx , hasher) ; } }
    };
}

impl_206!();