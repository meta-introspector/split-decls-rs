// Generated macro for make_issuer (function)
macro_rules! Depcrate_test_utilsmake_issuer {
() => {
// Module: crate::test_utils
// Provides: {"make_issuer"}
// Dependencies: {}
pub (crate) fn make_issuer (org_name : impl Into < String >) -> CertifiedIssuer < 'static , KeyPair > { let params = issuer_params (org_name) ; let key_pair = KeyPair :: generate_for (RCGEN_SIGNATURE_ALG) . unwrap () ; CertifiedIssuer :: self_signed (params , key_pair) . unwrap () }
};
}
