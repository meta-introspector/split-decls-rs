macro_rules! deps {
    () => {
        HashStableContext!();
        DefPathHash!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < CTX : HashStableContext > ToStableHashKey < CTX > for DefPathHash { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , _ : & CTX) -> DefPathHash { * self } }
    };
}

impl_129!()