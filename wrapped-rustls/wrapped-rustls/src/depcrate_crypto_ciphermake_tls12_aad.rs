// Generated macro for make_tls12_aad (function)
macro_rules! Depcrate_crypto_ciphermake_tls12_aad {
() => {
// Module: crate::crypto::cipher
// Provides: {"make_tls12_aad"}
// Dependencies: {}
# [doc = " Returns a TLS1.2 `additional_data` encoding."] # [doc = ""] # [doc = " See RFC5246 s6.2.3.3 for the `additional_data` definition."] # [inline] pub fn make_tls12_aad (seq : u64 , typ : ContentType , vers : ProtocolVersion , len : usize ,) -> [u8 ; TLS12_AAD_SIZE] { let mut out = [0 ; TLS12_AAD_SIZE] ; codec :: put_u64 (seq , & mut out [0 ..]) ; out [8] = typ . into () ; codec :: put_u16 (vers . into () , & mut out [9 ..]) ; codec :: put_u16 (len as u16 , & mut out [11 ..]) ; out }
};
}
