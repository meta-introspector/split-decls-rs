// Generated macro for CertificateList (struct)
macro_rules! Depcrate_crlCertificateList {
() => {
// Module: crate::crl
// Provides: {"CertificateList"}
// Dependencies: {}
# [doc = " `CertificateList` as defined in [RFC 5280 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " CertificateList  ::=  SEQUENCE  {"] # [doc = "     tbsCertList          TBSCertList,"] # [doc = "     signatureAlgorithm   AlgorithmIdentifier,"] # [doc = "     signatureValue       BIT STRING"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 5.1]: https://datatracker.ietf.org/doc/html/rfc5280#section-5.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct CertificateList < P : Profile = Rfc5280 > { pub tbs_cert_list : TbsCertList < P > , pub signature_algorithm : AlgorithmIdentifier , pub signature : BitString , }
};
}
