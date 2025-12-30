// Generated macro for RevokedCert (struct)
macro_rules! Depcrate_crlRevokedCert {
() => {
// Module: crate::crl
// Provides: {"RevokedCert"}
// Dependencies: {}
# [doc = " Implicit intermediate structure from the ASN.1 definition of `TBSCertList`."] # [doc = ""] # [doc = " This type is used for the `revoked_certificates` field of `TbsCertList`."] # [doc = " See [RFC 5280 Section 5.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " RevokedCert ::= SEQUENCE {"] # [doc = "     userCertificate         CertificateSerialNumber,"] # [doc = "     revocationDate          Time,"] # [doc = "     crlEntryExtensions      Extensions OPTIONAL"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 5.1]: https://datatracker.ietf.org/doc/html/rfc5280#section-5.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct RevokedCert < P : Profile = Rfc5280 > { pub serial_number : SerialNumber < P > , pub revocation_date : Time , pub crl_entry_extensions : Option < Extensions > , }
};
}
