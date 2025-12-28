macro_rules! deps {
    () => {
        HashStableContext!();
        ExpnId!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < CTX : HashStableContext > HashStable < CTX > for ExpnId { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { assert_default_hashing_controls (ctx , "ExpnId") ; let hash = if * self == ExpnId :: root () { Fingerprint :: ZERO } else { self . expn_hash () . 0 } ; hash . hash_stable (ctx , hasher) ; } }
    };
}

impl_88!()