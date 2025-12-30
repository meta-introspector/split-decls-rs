// Generated macro for impl_984 (impl)
macro_rules! Depcrate_crypto_ring_tls13impl_984 {
() => {
// Module: crate::crypto::ring::tls13
// Provides: {"impl_984"}
// Dependencies: {}
impl Hkdf for RingHkdf { fn extract_from_zero_ikm (& self , salt : Option < & [u8] >) -> Box < dyn HkdfExpander > { let zeroes = [0u8 ; OkmBlock :: MAX_LEN] ; let salt = match salt { Some (salt) => salt , None => & zeroes [.. self . 0 . len ()] , } ; Box :: new (RingHkdfExpander { alg : self . 0 , prk : hkdf :: Salt :: new (self . 0 , salt) . extract (& zeroes [.. self . 0 . len ()]) , }) } fn extract_from_secret (& self , salt : Option < & [u8] > , secret : & [u8]) -> Box < dyn HkdfExpander > { let zeroes = [0u8 ; OkmBlock :: MAX_LEN] ; let salt = match salt { Some (salt) => salt , None => & zeroes [.. self . 0 . len ()] , } ; Box :: new (RingHkdfExpander { alg : self . 0 , prk : hkdf :: Salt :: new (self . 0 , salt) . extract (secret) , }) } fn expander_for_okm (& self , okm : & OkmBlock) -> Box < dyn HkdfExpander > { Box :: new (RingHkdfExpander { alg : self . 0 , prk : hkdf :: Prk :: new_less_safe (self . 0 , okm . as_ref ()) , }) } fn hmac_sign (& self , key : & OkmBlock , message : & [u8]) -> crypto :: hmac :: Tag { crypto :: hmac :: Tag :: new (hmac :: sign (& hmac :: Key :: new (self . 1 , key . as_ref ()) , message) . as_ref ()) } fn fips (& self) -> bool { super :: fips () } }
};
}
