macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " SHA-224 hasher."] pub struct Sha224 (CtOutWrapper < block_api :: Sha256VarCore , U28 >) ; oid : "2.16.840.1.101.3.4.2.4" ; impl : FixedHashTraits ;) ;
    };
}

macro_50!()