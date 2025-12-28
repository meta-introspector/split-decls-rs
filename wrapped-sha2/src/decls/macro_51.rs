macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! macro_51 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " SHA-512/224 hasher."] pub struct Sha512_224 (CtOutWrapper < block_api :: Sha512VarCore , U28 >) ; oid : "2.16.840.1.101.3.4.2.5" ; impl : FixedHashTraits ;) ;
    };
}

macro_51!()