// Generated macro for Tls13AeadAlgorithm (trait)
macro_rules! Depcrate_crypto_cipherTls13AeadAlgorithm {
() => {
// Module: crate::crypto::cipher
// Provides: {"Tls13AeadAlgorithm"}
// Dependencies: {}
# [doc = " Factory trait for building `MessageEncrypter` and `MessageDecrypter` for a TLS1.3 cipher suite."] pub trait Tls13AeadAlgorithm : Send + Sync { # [doc = " Build a `MessageEncrypter` for the given key/iv."] fn encrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageEncrypter > ; # [doc = " Build a `MessageDecrypter` for the given key/iv."] fn decrypter (& self , key : AeadKey , iv : Iv) -> Box < dyn MessageDecrypter > ; # [doc = " The length of key in bytes required by `encrypter()` and `decrypter()`."] fn key_len (& self) -> usize ; # [doc = " The length of IV in bytes required by `encrypter()` and `decrypter()`."] fn iv_len (& self) -> usize { NONCE_LEN } # [doc = " Convert the key material from `key`/`iv`, into a `ConnectionTrafficSecrets` item."] # [doc = ""] # [doc = " May return [`UnsupportedOperationError`] if the AEAD algorithm is not a supported"] # [doc = " variant of `ConnectionTrafficSecrets`."] fn extract_keys (& self , key : AeadKey , iv : Iv ,) -> Result < ConnectionTrafficSecrets , UnsupportedOperationError > ; # [doc = " Return `true` if this is backed by a FIPS-approved implementation."] fn fips (& self) -> bool { false } }
};
}
