macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! macro_48 {
    () => {
        deps!();
        digest :: buffer_fixed ! (# [doc = " SHA-384 hasher."] pub struct Sha384 (CtOutWrapper < block_api :: Sha512VarCore , U48 >) ; oid : "2.16.840.1.101.3.4.2.2" ; impl : FixedHashTraits ;) ;
    };
}

macro_48!()