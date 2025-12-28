macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_530 {
    () => {
        deps!();
        impl < 'a , T , CTX > HashStable < CTX > for & 'a T where T : HashStable < CTX > + ? Sized , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
    };
}

impl_530!()