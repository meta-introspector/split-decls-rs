// Generated macro for BasicOcspResponse (struct)
macro_rules! Depcrate_basicBasicOcspResponse {
() => {
// Module: crate::basic
// Provides: {"BasicOcspResponse"}
// Dependencies: {}
# [doc = " BasicOcspResponse structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " BasicOCSPResponse ::= SEQUENCE {"] # [doc = "   tbsResponseData          ResponseData,"] # [doc = "   signatureAlgorithm       AlgorithmIdentifier,"] # [doc = "   signature                BIT STRING,"] # [doc = "   certs                [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct BasicOcspResponse { pub tbs_response_data : ResponseData , pub signature_algorithm : AlgorithmIdentifierOwned , pub signature : BitString , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub certs : Option < Vec < Certificate > > , }
};
}
