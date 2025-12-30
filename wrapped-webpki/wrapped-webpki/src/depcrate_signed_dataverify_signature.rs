// Generated macro for verify_signature (function)
macro_rules! Depcrate_signed_dataverify_signature {
() => {
// Module: crate::signed_data
// Provides: {"verify_signature"}
// Dependencies: {}
pub (crate) fn verify_signature (signature_alg : & dyn SignatureVerificationAlgorithm , spki_value : untrusted :: Input < '_ > , msg : untrusted :: Input < '_ > , signature : untrusted :: Input < '_ > ,) -> Result < () , Error > { let spki = der :: read_all :: < SubjectPublicKeyInfo < '_ > > (spki_value) ? ; if signature_alg . public_key_alg_id () . as_ref () != spki . algorithm_id_value . as_slice_less_safe () { return Err (Error :: UnsupportedSignatureAlgorithmForPublicKey (UnsupportedSignatureAlgorithmForPublicKeyContext { # [cfg (feature = "alloc")] signature_algorithm_id : signature_alg . signature_alg_id () . as_ref () . to_vec () , # [cfg (feature = "alloc")] public_key_algorithm_id : spki . algorithm_id_value . as_slice_less_safe () . to_vec () , } ,)) ; } signature_alg . verify_signature (spki . key_value . as_slice_less_safe () , msg . as_slice_less_safe () , signature . as_slice_less_safe () ,) . map_err (| _ | Error :: InvalidSignatureForPublicKey) }
};
}
