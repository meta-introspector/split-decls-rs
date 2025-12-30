// Generated macro for issuer_params (function)
macro_rules! Depcrate_test_utilsissuer_params {
() => {
// Module: crate::test_utils
// Provides: {"issuer_params"}
// Dependencies: {}
# [doc = " Populate a [CertificateParams] that describes an unconstrained issuer certificate capable"] # [doc = " of signing other certificates and CRLs, with the given `org_name` as an organization distinguished"] # [doc = " subject name."] pub (crate) fn issuer_params (org_name : impl Into < String >) -> rcgen :: CertificateParams { let mut ca_params = rcgen :: CertificateParams :: new (Vec :: new ()) . unwrap () ; ca_params . distinguished_name . push (rcgen :: DnType :: OrganizationName , org_name) ; ca_params . is_ca = rcgen :: IsCa :: Ca (rcgen :: BasicConstraints :: Unconstrained) ; ca_params . key_usages = vec ! [rcgen :: KeyUsagePurpose :: KeyCertSign , rcgen :: KeyUsagePurpose :: DigitalSignature , rcgen :: KeyUsagePurpose :: CrlSign ,] ; ca_params }
};
}
