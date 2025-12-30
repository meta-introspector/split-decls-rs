// Generated macro for impl_1061 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1061 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1061"}
// Dependencies: {}
impl < const KEY_SIZE : usize , const KDF_SIZE : usize > Opener < KEY_SIZE , KDF_SIZE > { # [doc = " See [RFC 9180 §5.1.1 \"Encryption to a Public Key\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-5.1.1"] fn new (suite : & HpkeAwsLcRs < KEY_SIZE , KDF_SIZE > , enc : & EncapsulatedSecret , info : & [u8] , secret_key : & HpkePrivateKey ,) -> Result < Self , Error > { Ok (Self { key_schedule : suite . key_schedule (suite . dh_kem . decap (enc , secret_key) ? , info) ? , }) } }
};
}
