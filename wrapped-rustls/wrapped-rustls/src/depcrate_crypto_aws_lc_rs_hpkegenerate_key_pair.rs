// Generated macro for generate_key_pair (function)
macro_rules! Depcrate_crypto_aws_lc_rs_hpkegenerate_key_pair {
() => {
// Module: crate::crypto::aws_lc_rs::hpke
// Provides: {"generate_key_pair"}
// Dependencies: {}
fn generate_key_pair (alg : & 'static agreement :: Algorithm ,) -> Result < (HpkePublicKey , agreement :: PrivateKey) , Error > { let private_key = agreement :: PrivateKey :: generate (alg) . map_err (unspecified_err) ? ; let public_key = HpkePublicKey (private_key . compute_public_key () . map_err (unspecified_err) ? . as_ref () . to_vec () ,) ; Ok ((public_key , private_key)) }
};
}
