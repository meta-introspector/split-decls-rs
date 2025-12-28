macro_rules! macro_5 {
    () => {
        digest :: buffer_fixed ! (# [doc = " SHA-256 hasher."] pub struct Sha256 (CtOutWrapper < block_api :: Sha256VarCore , U32 >) ; oid : "2.16.840.1.101.3.4.2.1" ; impl : FixedHashTraits ;) ;
    };
}

macro_5!()