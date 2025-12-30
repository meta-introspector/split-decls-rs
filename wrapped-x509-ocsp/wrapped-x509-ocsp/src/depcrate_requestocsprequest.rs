// Generated macro for OcspRequest (struct)
macro_rules! Depcrate_requestOcspRequest {
() => {
// Module: crate::request
// Provides: {"OcspRequest"}
// Dependencies: {}
# [doc = " OCSPRequest structure as defined in [RFC 6960 Section 4.1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " OCSPRequest ::= SEQUENCE {"] # [doc = "    tbsRequest              TBSRequest,"] # [doc = "    optionalSignature   [0] EXPLICIT Signature OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OcspRequest < P : Profile + 'static = Rfc5280 > { pub tbs_request : TbsRequest < P > , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub optional_signature : Option < Signature > , }
};
}
