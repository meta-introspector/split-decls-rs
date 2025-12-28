macro_rules! deps {
    () => {
        HashStable!();
        StableOrd!();
    };
}

macro_rules! impl_544 {
    () => {
        deps!();
        impl < K , HCX > HashStable < HCX > for :: std :: collections :: BTreeSet < K > where K : HashStable < HCX > + StableOrd , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . len () . hash_stable (hcx , hasher) ; for entry in self . iter () { entry . hash_stable (hcx , hasher) ; } } }
    };
}

impl_544!()