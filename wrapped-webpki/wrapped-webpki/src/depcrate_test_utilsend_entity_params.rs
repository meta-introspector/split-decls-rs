// Generated macro for end_entity_params (function)
macro_rules! Depcrate_test_utilsend_entity_params {
() => {
// Module: crate::test_utils
// Provides: {"end_entity_params"}
// Dependencies: {}
pub (crate) fn end_entity_params (subject_alt_names : Vec < String >) -> rcgen :: CertificateParams { let mut ee_params = rcgen :: CertificateParams :: new (subject_alt_names) . unwrap () ; ee_params . is_ca = rcgen :: IsCa :: ExplicitNoCa ; ee_params }
};
}
