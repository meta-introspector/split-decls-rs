macro_rules! deps {
    () => {
        Crc32Reader!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < R > Crc32Reader < R > { # [doc = " Get a new Crc32Reader which checks the inner reader against checksum."] # [doc = " The check is disabled if `ae2_encrypted == true`."] pub (crate) fn new (inner : R , checksum : u32 , ae2_encrypted : bool) -> Crc32Reader < R > { Crc32Reader { inner , hasher : Hasher :: new () , check : checksum , enabled : ! ae2_encrypted , } } fn check_matches (& self) -> bool { self . check == self . hasher . clone () . finalize () } pub fn into_inner (self) -> R { self . inner } }
    };
}

impl_51!();