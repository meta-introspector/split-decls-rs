// Generated macro for OcspResponse (struct)
macro_rules! Depcrate_responseOcspResponse {
() => {
// Module: crate::response
// Provides: {"OcspResponse"}
// Dependencies: {}
# [doc = " OCSPResponse structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " OCSPResponse ::= SEQUENCE {"] # [doc = "    responseStatus          OCSPResponseStatus,"] # [doc = "    responseBytes           [0] EXPLICIT ResponseBytes OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct OcspResponse { pub response_status : OcspResponseStatus , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub response_bytes : Option < ResponseBytes > , }
};
}
