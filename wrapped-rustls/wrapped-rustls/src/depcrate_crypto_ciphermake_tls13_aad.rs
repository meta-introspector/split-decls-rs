// Generated macro for make_tls13_aad (function)
macro_rules! Depcrate_crypto_ciphermake_tls13_aad {
() => {
// Module: crate::crypto::cipher
// Provides: {"make_tls13_aad"}
// Dependencies: {}
# [doc = " Returns a TLS1.3 `additional_data` encoding."] # [doc = ""] # [doc = " See RFC8446 s5.2 for the `additional_data` definition."] # [inline] pub fn make_tls13_aad (payload_len : usize) -> [u8 ; 5] { let version = ProtocolVersion :: TLSv1_2 . to_array () ; [ContentType :: ApplicationData . into () , version [0] , version [1] , (payload_len >> 8) as u8 , (payload_len & 0xff) as u8 ,] }
};
}
