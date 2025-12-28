macro_rules! deps {
    () => {
        Fingerprint!();
        HashStable!();
    };
}

macro_rules! hash_iter_order_independent {
    () => {
        deps!();
        fn hash_iter_order_independent < HCX , T : HashStable < HCX > , I : Iterator < Item = T > + ExactSizeIterator , > (mut it : I , hcx : & mut HCX , hasher : & mut StableHasher ,) { let len = it . len () ; len . hash_stable (hcx , hasher) ; match len { 0 => { } 1 => { it . next () . unwrap () . hash_stable (hcx , hasher) ; } _ => { let mut accumulator = Fingerprint :: ZERO ; for item in it { let mut item_hasher = StableHasher :: new () ; item . hash_stable (hcx , & mut item_hasher) ; let item_fingerprint : Fingerprint = item_hasher . finish () ; accumulator = accumulator . combine_commutative (item_fingerprint) ; } accumulator . hash_stable (hcx , hasher) ; } } }
    };
}

hash_iter_order_independent!()