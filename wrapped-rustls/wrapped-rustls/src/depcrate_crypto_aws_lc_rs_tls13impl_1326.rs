// Generated macro for impl_1326 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_tls13impl_1326 {
() => {
// Module: crate::crypto::aws_lc_rs::tls13
// Provides: {"impl_1326"}
// Dependencies: {}
impl Hkdf for AwsLcHkdf { fn extract_from_zero_ikm (& self , salt : Option < & [u8] >) -> Box < dyn HkdfExpander > { let zeroes = [0u8 ; OkmBlock :: MAX_LEN] ; let salt = match salt { Some (salt) => salt , None => & zeroes [.. self . 0 . len ()] , } ; Box :: new (AwsLcHkdfExpander { alg : self . 0 , prk : hkdf :: Salt :: new (self . 0 , salt) . extract (& zeroes [.. self . 0 . len ()]) , }) } fn extract_from_secret (& self , salt : Option < & [u8] > , secret : & [u8]) -> Box < dyn HkdfExpander > { let zeroes = [0u8 ; OkmBlock :: MAX_LEN] ; let salt = match salt { Some (salt) => salt , None => & zeroes [.. self . 0 . len ()] , } ; Box :: new (AwsLcHkdfExpander { alg : self . 0 , prk : hkdf :: Salt :: new (self . 0 , salt) . extract (secret) , }) } fn expander_for_okm (& self , okm : & OkmBlock) -> Box < dyn HkdfExpander > { Box :: new (AwsLcHkdfExpander { alg : self . 0 , prk : hkdf :: Prk :: new_less_safe (self . 0 , okm . as_ref ()) , }) } fn hmac_sign (& self , key : & OkmBlock , message : & [u8]) -> crypto :: hmac :: Tag { crypto :: hmac :: Tag :: new (hmac :: sign (& hmac :: Key :: new (self . 1 , key . as_ref ()) , message) . as_ref ()) } fn fips (& self) -> bool { super :: fips () } }
};
}
