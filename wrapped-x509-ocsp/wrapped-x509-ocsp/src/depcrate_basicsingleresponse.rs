// Generated macro for SingleResponse (struct)
macro_rules! Depcrate_basicSingleResponse {
() => {
// Module: crate::basic
// Provides: {"SingleResponse"}
// Dependencies: {}
# [doc = " SingleResponse structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " SingleResponse ::= SEQUENCE {"] # [doc = "    certID                  CertID,"] # [doc = "    certStatus              CertStatus,"] # [doc = "    thisUpdate              GeneralizedTime,"] # [doc = "    nextUpdate              [0] EXPLICIT GeneralizedTime OPTIONAL,"] # [doc = "    singleExtensions        [1] EXPLICIT Extensions OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct SingleResponse { pub cert_id : CertId , pub cert_status : CertStatus , pub this_update : OcspGeneralizedTime , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub next_update : Option < OcspGeneralizedTime > , # [asn1 (context_specific = "1" , optional = "true" , tag_mode = "EXPLICIT")] pub single_extensions : Option < Extensions > , }
};
}
