// Generated macro for KeyHash (type)
macro_rules! Depcrate_responder_idKeyHash {
() => {
// Module: crate::responder_id
// Provides: {"KeyHash"}
// Dependencies: {}
# [doc = " KeyHash structure as defined in [RFC 6960 Section 4.2.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " KeyHash ::= OCTET STRING -- SHA-1 hash of responder's public key"] # [doc = "                          -- (i.e., the SHA-1 hash of the value of the"] # [doc = "                          -- BIT STRING subjectPublicKey [excluding"] # [doc = "                          -- the tag, length, and number of unused"] # [doc = "                          -- bits] in the responder's certificate)"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.2.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.2.1"] pub type KeyHash = OctetString ;
};
}
