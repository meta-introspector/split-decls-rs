// Generated macro for impl_104 (impl)
macro_rules! Depcrate_end_entityimpl_104 {
() => {
// Module: crate::end_entity
// Provides: {"impl_104"}
// Dependencies: {}
impl < 'a > TryFrom < & 'a CertificateDer < 'a > > for EndEntityCert < 'a > { type Error = Error ; # [doc = " Parse the ASN.1 DER-encoded X.509 encoding of the certificate"] # [doc = " `cert_der`."] fn try_from (cert : & 'a CertificateDer < 'a >) -> Result < Self , Self :: Error > { Ok (Self { inner : cert :: Cert :: from_der (untrusted :: Input :: from (cert . as_ref ())) ? , }) } }
};
}
