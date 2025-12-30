// Generated macro for HpkeAwsLcRs (struct)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeHpkeAwsLcRs {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"HpkeAwsLcRs"}
// Dependencies: {}
# [doc = " `HpkeAwsLcRs` holds the concrete instantiations of the algorithms specified by the [HpkeSuite]."] pub struct HpkeAwsLcRs < const KEY_SIZE : usize , const KDF_SIZE : usize > { suite : HpkeSuite , dh_kem : & 'static DhKem < KDF_SIZE > , hkdf : & 'static dyn HkdfPrkExtract , aead : & 'static aead :: Algorithm , }
};
}
