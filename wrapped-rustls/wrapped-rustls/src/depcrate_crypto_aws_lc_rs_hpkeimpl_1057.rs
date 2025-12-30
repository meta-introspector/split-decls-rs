// Generated macro for impl_1057 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1057 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1057"}
// Dependencies: {}
impl < const KEY_SIZE : usize , const KDF_SIZE : usize > Sealer < KEY_SIZE , KDF_SIZE > { # [doc = " See [RFC 9180 §5.1.1 \"Encryption to a Public Key\"][0]."] # [doc = ""] # [doc = " [0]: https://www.rfc-editor.org/rfc/rfc9180.html#section-5.1.1"] fn new (suite : & HpkeAwsLcRs < KEY_SIZE , KDF_SIZE > , info : & [u8] , pub_key : & HpkePublicKey ,) -> Result < (EncapsulatedSecret , Self) , Error > { let (shared_secret , enc) = suite . dh_kem . encap (pub_key) ? ; let key_schedule = suite . key_schedule (shared_secret , info) ? ; Ok ((enc , Self { key_schedule })) } # [doc = " A **test only** constructor that uses a pre-specified ephemeral agreement private key"] # [doc = " instead of one that is randomly generated."] # [cfg (test)] fn test_only_new (suite : & HpkeAwsLcRs < KEY_SIZE , KDF_SIZE > , info : & [u8] , pub_key : & HpkePublicKey , sk_e : & [u8] ,) -> Result < (EncapsulatedSecret , Self) , Error > { let (shared_secret , enc) = suite . dh_kem . test_only_encap (pub_key , sk_e) ? ; let key_schedule = suite . key_schedule (shared_secret , info) ? ; Ok ((enc , Self { key_schedule })) } }
};
}
