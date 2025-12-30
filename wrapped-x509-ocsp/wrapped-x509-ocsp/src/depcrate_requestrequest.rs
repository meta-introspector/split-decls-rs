// Generated macro for Request (struct)
macro_rules! Depcrate_requestRequest {
() => {
// Module: crate::request
// Provides: {"Request"}
// Dependencies: {}
# [doc = " Request structure as defined in [RFC 6960 Section 4.1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " Request ::= SEQUENCE {"] # [doc = "    reqCert                     CertID,"] # [doc = "    singleRequestExtensions     [0] EXPLICIT Extensions OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct Request < P : Profile + 'static = Rfc5280 > { pub req_cert : CertId < P > , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub single_request_extensions : Option < Extensions > , }
};
}
