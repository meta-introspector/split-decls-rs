macro_rules! deps {
    () => {
        ToStableHashKey!();
    };
}

macro_rules! impl_523 {
    () => {
        deps!();
        impl < HCX > ToStableHashKey < HCX > for String { type KeyType = String ; # [inline] fn to_stable_hash_key (& self , _ : & HCX) -> Self :: KeyType { self . clone () } }
    };
}

impl_523!();