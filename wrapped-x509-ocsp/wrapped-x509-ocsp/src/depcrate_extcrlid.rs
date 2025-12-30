// Generated macro for CrlId (struct)
macro_rules! Depcrate_extCrlId {
() => {
// Module: crate::ext
// Provides: {"CrlId"}
// Dependencies: {}
# [doc = " CrlID structure as defined in [RFC 6960 Section 4.4.2]."] # [doc = ""] # [doc = " ```text"] # [doc = " CrlID ::= SEQUENCE {"] # [doc = "     crlUrl               [0] EXPLICIT IA5String OPTIONAL,"] # [doc = "     crlNum               [1] EXPLICIT INTEGER OPTIONAL,"] # [doc = "     crlTime              [2] EXPLICIT GeneralizedTime OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.4.2]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.4.2"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct CrlId { # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub crl_url : Option < Ia5String > , # [asn1 (context_specific = "1" , optional = "true" , tag_mode = "EXPLICIT")] pub crl_num : Option < Uint > , # [asn1 (context_specific = "2" , optional = "true" , tag_mode = "EXPLICIT")] pub crl_time : Option < OcspGeneralizedTime > , }
};
}
