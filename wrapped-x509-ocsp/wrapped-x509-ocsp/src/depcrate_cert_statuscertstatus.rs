// Generated macro for CertStatus (enum)
macro_rules! Depcrate_cert_statusCertStatus {
() => {
// Module: crate::cert_status
// Provides: {"CertStatus"}
// Dependencies: {}
# [doc = " CertStatus structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " CertStatus ::= CHOICE {"] # [doc = "    good                [0] IMPLICIT NULL,"] # [doc = "    revoked             [1] IMPLICIT RevokedInfo,"] # [doc = "    unknown             [2] IMPLICIT UnknownInfo }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum CertStatus { # [asn1 (context_specific = "0" , tag_mode = "IMPLICIT")] Good (Null) , # [asn1 (context_specific = "1" , tag_mode = "IMPLICIT" , constructed = "true")] Revoked (RevokedInfo) , # [asn1 (context_specific = "2" , tag_mode = "IMPLICIT")] Unknown (UnknownInfo) , }
};
}
