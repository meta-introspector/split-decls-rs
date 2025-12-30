// Generated macro for ResponseData (struct)
macro_rules! Depcrate_basicResponseData {
() => {
// Module: crate::basic
// Provides: {"ResponseData"}
// Dependencies: {}
# [doc = " ResponseData structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " ResponseData ::= SEQUENCE {"] # [doc = "    version              [0] EXPLICIT Version DEFAULT v1,"] # [doc = "    responderID             ResponderID,"] # [doc = "    producedAt              GeneralizedTime,"] # [doc = "    responses               SEQUENCE OF SingleResponse,"] # [doc = "    responseExtensions   [1] EXPLICIT Extensions OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ResponseData { # [asn1 (context_specific = "0" , default = "Default::default" , tag_mode = "EXPLICIT")] pub version : Version , pub responder_id : ResponderId , pub produced_at : OcspGeneralizedTime , pub responses : Vec < SingleResponse > , # [asn1 (context_specific = "1" , optional = "true" , tag_mode = "EXPLICIT")] pub response_extensions : Option < Extensions > , }
};
}
