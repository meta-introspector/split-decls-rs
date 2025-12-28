macro_rules! deps {
    () => {
        DefId!();
        SimplifiedType!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        # [cfg (feature = "nightly")] impl < HCX : Clone , DefId : HashStable < HCX > > ToStableHashKey < HCX > for SimplifiedType < DefId > { type KeyType = Fingerprint ; # [inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Fingerprint { let mut hasher = StableHasher :: new () ; let mut hcx : HCX = hcx . clone () ; self . hash_stable (& mut hcx , & mut hasher) ; hasher . finish () } }
    };
}

impl_40!();