// Generated macro for RevokedInfo (struct)
macro_rules! Depcrate_cert_statusRevokedInfo {
() => {
// Module: crate::cert_status
// Provides: {"RevokedInfo"}
// Dependencies: {}
# [doc = " RevokedInfo structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " RevokedInfo ::= SEQUENCE {"] # [doc = "    revocationTime          GeneralizedTime,"] # [doc = "    revocationReason        [0] EXPLICIT CRLReason OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Copy , Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct RevokedInfo { pub revocation_time : OcspGeneralizedTime , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub revocation_reason : Option < CrlReason > , }
};
}
