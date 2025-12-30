// Generated macro for TbsRequest (struct)
macro_rules! Depcrate_requestTbsRequest {
() => {
// Module: crate::request
// Provides: {"TbsRequest"}
// Dependencies: {}
# [doc = " TBSRequest structure as defined in [RFC 6960 Section 4.1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " TBSRequest ::= SEQUENCE {"] # [doc = "    version             [0] EXPLICIT Version DEFAULT v1,"] # [doc = "    requestorName       [1] EXPLICIT GeneralName OPTIONAL,"] # [doc = "    requestList             SEQUENCE OF Request,"] # [doc = "    requestExtensions   [2] EXPLICIT Extensions OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1"] # [derive (Clone , Debug , Default , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct TbsRequest < P : Profile + 'static = Rfc5280 > { # [asn1 (context_specific = "0" , default = "Default::default" , tag_mode = "EXPLICIT")] pub version : Version , # [asn1 (context_specific = "1" , optional = "true" , tag_mode = "EXPLICIT")] pub requestor_name : Option < GeneralName > , pub request_list : Vec < Request < P > > , # [asn1 (context_specific = "2" , optional = "true" , tag_mode = "EXPLICIT")] pub request_extensions : Option < Extensions > , }
};
}
