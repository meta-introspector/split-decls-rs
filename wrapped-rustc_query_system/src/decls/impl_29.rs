macro_rules! deps {
    () => {
        WorkProductId!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < HCX > ToStableHashKey < HCX > for WorkProductId { type KeyType = Fingerprint ; # [inline] fn to_stable_hash_key (& self , _ : & HCX) -> Self :: KeyType { self . hash } }
    };
}

impl_29!()