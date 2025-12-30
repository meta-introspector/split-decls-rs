// Generated macro for impl_1075 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkeimpl_1075 {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"impl_1075"}
// Dependencies: {}
impl < const KEY_SIZE : usize > NonceSequence for & mut KeySchedule < KEY_SIZE > { fn advance (& mut self) -> Result < Nonce , aws_lc_rs :: error :: Unspecified > { let nonce = self . compute_nonce () ; self . increment_seq_num () ? ; Nonce :: try_assume_unique_for_key (& nonce) } }
};
}
