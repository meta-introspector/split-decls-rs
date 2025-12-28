macro_rules! deps {
    () => {
        HashStableContext!();
        DefId!();
        DefPathHash!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < CTX : HashStableContext > ToStableHashKey < CTX > for DefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (* self) } }
    };
}

impl_126!()