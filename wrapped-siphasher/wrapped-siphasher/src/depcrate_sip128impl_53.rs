// Generated macro for impl_53 (impl)
macro_rules! Depcrate_sip128impl_53 {
() => {
// Module: crate::sip128
// Provides: {"impl_53"}
// Dependencies: {}
impl SipHasher13 { # [doc = " Creates a new `SipHasher13` with the two initial keys set to 0."] # [inline] pub fn new () -> SipHasher13 { SipHasher13 :: new_with_keys (0 , 0) } # [doc = " Creates a `SipHasher13` that is keyed off the provided keys."] # [inline] pub fn new_with_keys (key0 : u64 , key1 : u64) -> SipHasher13 { SipHasher13 { hasher : Hasher :: new_with_keys (key0 , key1) , } } # [doc = " Creates a `SipHasher13` from a 16 byte key."] pub fn new_with_key (key : & [u8 ; 16]) -> SipHasher13 { let mut b0 = [0u8 ; 8] ; let mut b1 = [0u8 ; 8] ; b0 . copy_from_slice (& key [0 .. 8]) ; b1 . copy_from_slice (& key [8 .. 16]) ; let key0 = u64 :: from_le_bytes (b0) ; let key1 = u64 :: from_le_bytes (b1) ; Self :: new_with_keys (key0 , key1) } # [doc = " Get the keys used by this hasher"] pub fn keys (& self) -> (u64 , u64) { (self . hasher . k0 , self . hasher . k1) } # [doc = " Get the key used by this hasher as a 16 byte vector"] pub fn key (& self) -> [u8 ; 16] { let mut bytes = [0u8 ; 16] ; bytes [0 .. 8] . copy_from_slice (& self . hasher . k0 . to_le_bytes ()) ; bytes [8 .. 16] . copy_from_slice (& self . hasher . k1 . to_le_bytes ()) ; bytes } # [doc = " Hash a byte array - This is the easiest and safest way to use SipHash."] # [inline] pub fn hash (& self , bytes : & [u8]) -> Hash128 { let mut hasher = self . hasher ; hasher . write (bytes) ; hasher . finish128 () } }
};
}
