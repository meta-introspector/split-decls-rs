macro_rules! deps {
    () => {
        DefPathHash!();
        HashStableContext!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < CTX : HashStableContext > ToStableHashKey < CTX > for CrateNum { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { self . as_def_id () . to_stable_hash_key (hcx) } }
    };
}

impl_128!()