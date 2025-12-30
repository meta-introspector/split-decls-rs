// Generated macro for DhKem (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeDhKem {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"DhKem"}
// Dependencies: {}
# [doc = " A Diffie-Hellman (DH) based Key Encapsulation Mechanism (KEM)."] # [doc = ""] # [doc = " See [RFC 9180 §4.1 \"DH-Based KEM (DHKEM)\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-4.1"] struct DhKem < const KDF_SIZE : usize > { id : HpkeKem , agreement_algorithm : & 'static agreement :: Algorithm , key_generator : & 'static (dyn Fn () -> Result < (HpkePublicKey , HpkePrivateKey) , Error > + Send + Sync) , hkdf : & 'static dyn HkdfPrkExtract , }
};
}
