// Generated macro for TbsCertList (struct)
macro_rules! Depcrate_crlTbsCertList {
() => {
// Module: crate::crl
// Provides: {"TbsCertList"}
// Dependencies: {}
# [doc = " `TbsCertList` as defined in [RFC 5280 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " TBSCertList  ::=  SEQUENCE  {"] # [doc = "      version                 Version OPTIONAL, -- if present, MUST be v2"] # [doc = "      signature               AlgorithmIdentifier,"] # [doc = "      issuer                  Name,"] # [doc = "      thisUpdate              Time,"] # [doc = "      nextUpdate              Time OPTIONAL,"] # [doc = "      revokedCertificates     SEQUENCE OF SEQUENCE  {"] # [doc = "           userCertificate         CertificateSerialNumber,"] # [doc = "           revocationDate          Time,"] # [doc = "           crlEntryExtensions      Extensions OPTIONAL -- if present, version MUST be v2"] # [doc = "      }  OPTIONAL,"] # [doc = "      crlExtensions           [0]  EXPLICIT Extensions OPTIONAL -- if present, version MUST be v2"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 5.1]: https://datatracker.ietf.org/doc/html/rfc5280#section-5.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct TbsCertList < P : Profile = Rfc5280 > { pub version : Version , pub signature : AlgorithmIdentifier , pub issuer : Name , pub this_update : Time , pub next_update : Option < Time > , pub revoked_certificates : Option < Vec < RevokedCert < P > > > , # [asn1 (context_specific = "0" , tag_mode = "EXPLICIT" , optional = "true")] pub crl_extensions : Option < Extensions > , }
};
}
