macro_rules! deps {
    () => {
        Tag!();
        TaggedRef!();
        HashStable!();
        Aligned!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl < 'a , P , T , HCX > HashStable < HCX > for TaggedRef < 'a , P , T > where P : HashStable < HCX > + Aligned + ? Sized , T : Tag + HashStable < HCX > , { fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . pointer () . hash_stable (hcx , hasher) ; self . tag () . hash_stable (hcx , hasher) ; } }
    };
}

impl_589!();