// Generated macro for make_end_entity (function)
macro_rules! Depcrate_test_utilsmake_end_entity {
() => {
// Module: crate::test_utils
// Provides: {"make_end_entity"}
// Dependencies: {}
# [cfg_attr (not (feature = "ring") , allow (dead_code))] pub (crate) fn make_end_entity (issuer : & Issuer < '_ , impl SigningKey >) -> CertifiedKey < KeyPair > { let signing_key = KeyPair :: generate_for (RCGEN_SIGNATURE_ALG) . unwrap () ; CertifiedKey { cert : end_entity_params (vec ! ["example.com" . into ()]) . signed_by (& signing_key , issuer) . unwrap () , signing_key , } }
};
}
