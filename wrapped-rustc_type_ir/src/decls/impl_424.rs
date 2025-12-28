macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_424 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < T : HashStable < CTX > , CTX > HashStable < CTX > for WithCachedTypeInfo < T > { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { if self . stable_hash == Fingerprint :: ZERO || cfg ! (debug_assertions) { let stable_hash : Fingerprint = { let mut hasher = StableHasher :: new () ; self . internee . hash_stable (hcx , & mut hasher) ; hasher . finish () } ; if cfg ! (debug_assertions) && self . stable_hash != Fingerprint :: ZERO { assert_eq ! (stable_hash , self . stable_hash , "cached stable hash does not match freshly computed stable hash") ; } stable_hash . hash_stable (hcx , hasher) ; } else { self . stable_hash . hash_stable (hcx , hasher) ; } } }
    };
}

impl_424!();