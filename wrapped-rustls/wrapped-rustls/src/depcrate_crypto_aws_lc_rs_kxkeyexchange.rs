// Generated macro for KeyExchange (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_kxKeyExchange {
() => {
// Module: crate::crypto::aws_lc_rs::kx
// Provides: {"KeyExchange"}
// Dependencies: {}
# [doc = " An in-progress key exchange.  This has the algorithm,"] # [doc = " our private key, and our public key."] struct KeyExchange { name : NamedGroup , agreement_algorithm : & 'static agreement :: Algorithm , priv_key : agreement :: EphemeralPrivateKey , pub_key : agreement :: PublicKey , pub_key_validator : fn (& [u8]) -> bool , }
};
}
