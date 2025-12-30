// Generated macro for KeyBlockShape (struct)
macro_rules! Depcrate_crypto_cipherKeyBlockShape {
() => {
// Module: crate::crypto::cipher
// Provides: {"KeyBlockShape"}
// Dependencies: {}
# [doc = " How a TLS1.2 `key_block` is partitioned."] # [doc = ""] # [doc = " Note: ciphersuites with non-zero `mac_key_length` are  not currently supported."] # [expect (clippy :: exhaustive_structs)] pub struct KeyBlockShape { # [doc = " How long keys are."] # [doc = ""] # [doc = " `enc_key_length` terminology is from the standard ([RFC5246 A.6])."] # [doc = ""] # [doc = " [RFC5246 A.6]: <https://www.rfc-editor.org/rfc/rfc5246#appendix-A.6>"] pub enc_key_len : usize , # [doc = " How long the fixed part of the 'IV' is."] # [doc = ""] # [doc = " `fixed_iv_length` terminology is from the standard ([RFC5246 A.6])."] # [doc = ""] # [doc = " This isn't usually an IV, but we continue the"] # [doc = " terminology misuse to match the standard."] # [doc = ""] # [doc = " [RFC5246 A.6]: <https://www.rfc-editor.org/rfc/rfc5246#appendix-A.6>"] pub fixed_iv_len : usize , # [doc = " This is a non-standard extension which extends the"] # [doc = " key block to provide an initial explicit nonce offset,"] # [doc = " in a deterministic and safe way.  GCM needs this,"] # [doc = " chacha20poly1305 works this way by design."] pub explicit_nonce_len : usize , }
};
}
