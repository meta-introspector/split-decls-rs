// Generated macro for Cert (struct)
macro_rules! Depcrate_certCert {
() => {
// Module: crate::cert
// Provides: {"Cert"}
// Dependencies: {}
# [doc = " A parsed X509 certificate."] pub struct Cert < 'a > { pub (crate) serial : untrusted :: Input < 'a > , pub (crate) signed_data : SignedData < 'a > , pub (crate) issuer : untrusted :: Input < 'a > , pub (crate) validity : untrusted :: Input < 'a > , pub (crate) subject : untrusted :: Input < 'a > , pub (crate) spki : untrusted :: Input < 'a > , pub (crate) basic_constraints : Option < untrusted :: Input < 'a > > , pub (crate) key_usage : Option < untrusted :: Input < 'a > > , pub (crate) eku : Option < untrusted :: Input < 'a > > , pub (crate) name_constraints : Option < untrusted :: Input < 'a > > , pub (crate) subject_alt_name : Option < untrusted :: Input < 'a > > , pub (crate) crl_distribution_points : Option < untrusted :: Input < 'a > > , der : CertificateDer < 'a > , }
};
}
