macro_rules! deps {
    () => {
        ToStableHashKey!();
    };
}

macro_rules! impl_524 {
    () => {
        deps!();
        impl < HCX , T1 : ToStableHashKey < HCX > , T2 : ToStableHashKey < HCX > > ToStableHashKey < HCX > for (T1 , T2) { type KeyType = (T1 :: KeyType , T2 :: KeyType) ; # [inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType { (self . 0 . to_stable_hash_key (hcx) , self . 1 . to_stable_hash_key (hcx)) } }
    };
}

impl_524!()