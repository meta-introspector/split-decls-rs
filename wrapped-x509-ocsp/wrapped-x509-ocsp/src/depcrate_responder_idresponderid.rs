// Generated macro for ResponderId (enum)
macro_rules! Depcrate_responder_idResponderId {
() => {
// Module: crate::responder_id
// Provides: {"ResponderId"}
// Dependencies: {}
# [doc = " ResponderID structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " ResponderID ::= CHOICE {"] # [doc = "    byName              [1] Name,"] # [doc = "    byKey               [2] KeyHash }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] # [derive (Clone , Debug , Eq , PartialEq , Choice)] # [allow (missing_docs)] pub enum ResponderId { # [asn1 (context_specific = "1" , tag_mode = "EXPLICIT" , constructed = "true")] ByName (Name) , # [asn1 (context_specific = "2" , tag_mode = "EXPLICIT" , constructed = "true")] ByKey (KeyHash) , }
};
}
