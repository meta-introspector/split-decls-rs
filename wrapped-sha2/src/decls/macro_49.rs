macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! macro_49 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " SHA-512 hasher."] pub struct Sha512 (CtOutWrapper < block_api :: Sha512VarCore , U64 >) ; oid : "2.16.840.1.101.3.4.2.3" ; impl : FixedHashTraits ;) ;
    };
}

macro_49!();