// Generated macro for ResponseBytes (struct)
macro_rules! Depcrate_responseResponseBytes {
() => {
// Module: crate::response
// Provides: {"ResponseBytes"}
// Dependencies: {}
# [doc = " ResponseBytes structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " ResponseBytes ::= SEQUENCE {"] # [doc = "    responseType            OBJECT IDENTIFIER,"] # [doc = "    response                OCTET STRING }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct ResponseBytes { pub response_type : ObjectIdentifier , pub response : OctetString , }
};
}
