// Generated macro for CertificateInner (struct)
macro_rules! Depcrate_certificateCertificateInner {
() => {
// Module: crate::certificate
// Provides: {"CertificateInner"}
// Dependencies: {}
# [doc = " X.509 certificates are defined in [RFC 5280 Section 4.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " Certificate  ::=  SEQUENCE  {"] # [doc = "     tbsCertificate       TBSCertificate,"] # [doc = "     signatureAlgorithm   AlgorithmIdentifier,"] # [doc = "     signature            BIT STRING"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct CertificateInner < P : Profile = Rfc5280 > { pub (crate) tbs_certificate : TbsCertificateInner < P > , pub (crate) signature_algorithm : AlgorithmIdentifier , pub (crate) signature : BitString , }
};
}
