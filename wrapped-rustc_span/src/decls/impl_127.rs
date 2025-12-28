macro_rules! deps {
    () => {
        LocalDefId!();
        DefPathHash!();
        HashStableContext!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < CTX : HashStableContext > ToStableHashKey < CTX > for LocalDefId { type KeyType = DefPathHash ; # [inline] fn to_stable_hash_key (& self , hcx : & CTX) -> DefPathHash { hcx . def_path_hash (self . to_def_id ()) } }
    };
}

impl_127!();