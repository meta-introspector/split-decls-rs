macro_rules! deps {
    () => {
        StableOrd!();
        HashStable!();
    };
}

macro_rules! impl_543 {
    () => {
        deps!();
        impl < K , V , HCX > HashStable < HCX > for :: std :: collections :: BTreeMap < K , V > where K : HashStable < HCX > + StableOrd , V : HashStable < HCX > , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . len () . hash_stable (hcx , hasher) ; for entry in self . iter () { entry . hash_stable (hcx , hasher) ; } } }
    };
}

impl_543!()