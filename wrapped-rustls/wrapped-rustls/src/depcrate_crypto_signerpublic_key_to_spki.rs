// Generated macro for public_key_to_spki (function)
macro_rules! Depcrate_crypto_signerpublic_key_to_spki {
() => {
// Module: crate::crypto::signer
// Provides: {"public_key_to_spki"}
// Dependencies: {}
# [doc = " Convert a public key and algorithm identifier into [`SubjectPublicKeyInfoDer`]."] # [doc = ""] # [doc = " In the returned encoding, `alg_id` is used as the `algorithm` field, and `public_key` is"] # [doc = " wrapped inside an ASN.1 `BIT STRING` and then used as the `subjectPublicKey` field."] pub fn public_key_to_spki (alg_id : & AlgorithmIdentifier , public_key : impl AsRef < [u8] > ,) -> SubjectPublicKeyInfoDer < 'static > { let mut spki_inner = x509 :: wrap_in_sequence (alg_id . as_ref ()) ; spki_inner . extend (& x509 :: wrap_in_bit_string (public_key . as_ref ())) ; let spki = x509 :: wrap_in_sequence (& spki_inner) ; SubjectPublicKeyInfoDer :: from (spki) }
};
}
