// Generated macro for impl_1495 (impl)
macro_rules! Depcrate_crypto_tls13impl_1495 {
() => {
// Module: crate::crypto::tls13
// Provides: {"impl_1495"}
// Dependencies: {}
impl HkdfPrkExtract for HkdfUsingHmac < '_ > { fn extract_prk_from_secret (& self , salt : Option < & [u8] > , secret : & [u8]) -> Vec < u8 > { let zeroes = [0u8 ; hmac :: Tag :: MAX_LEN] ; let salt = match salt { Some (salt) => salt , None => & zeroes [.. self . 0 . hash_output_len ()] , } ; self . 0 . with_key (salt) . sign (& [secret]) . as_ref () . to_vec () } }
};
}
