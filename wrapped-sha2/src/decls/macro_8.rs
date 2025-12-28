macro_rules! macro_8 {
    () => {
        digest :: buffer_fixed ! (# [doc = " SHA-224 hasher."] pub struct Sha224 (CtOutWrapper < block_api :: Sha256VarCore , U28 >) ; oid : "2.16.840.1.101.3.4.2.4" ; impl : FixedHashTraits ;) ;
    };
}

macro_8!()