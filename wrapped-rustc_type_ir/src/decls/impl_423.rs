macro_rules! deps {
    () => {
        WithCachedTypeInfo!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl < T : Hash > Hash for WithCachedTypeInfo < T > { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { # [cfg (feature = "nightly")] if self . stable_hash != Fingerprint :: ZERO { return self . stable_hash . hash (s) ; } self . internee . hash (s) } }
    };
}

impl_423!();