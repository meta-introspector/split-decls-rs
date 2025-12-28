macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! hash_result {
    () => {
        deps!();
        pub fn hash_result < R > (hcx : & mut StableHashingContext < '_ > , result : & R) -> Fingerprint where R : for < 'a > HashStable < StableHashingContext < 'a > > , { let mut stable_hasher = StableHasher :: new () ; result . hash_stable (hcx , & mut stable_hasher) ; stable_hasher . finish () }
    };
}

hash_result!()