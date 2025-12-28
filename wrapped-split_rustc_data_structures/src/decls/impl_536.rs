macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_536 {
    () => {
        deps!();
        impl < R : Idx , C : Idx , CTX > HashStable < CTX > for bit_set :: BitMatrix < R , C > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
    };
}

impl_536!();