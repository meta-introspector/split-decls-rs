// Generated macro for load_key (function)
macro_rules! Depcrate_crypto_aws_lc_rs_signload_key {
() => {
// Module: crate::crypto::aws_lc_rs::sign
// Provides: {"load_key"}
// Dependencies: {}
# [cfg (any (test , bench))] fn load_key (provider : & CryptoProvider , der : PrivateKeyDer < 'static > ,) -> Result < Box < dyn SigningKey > , Error > { provider . key_provider . load_private_key (der) }
};
}
