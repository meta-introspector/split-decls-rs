// Generated macro for PreferredSignatureAlgorithm (struct)
macro_rules! Depcrate_extPreferredSignatureAlgorithm {
() => {
// Module: crate::ext
// Provides: {"PreferredSignatureAlgorithm"}
// Dependencies: {}
# [doc = " PreferredSignatureAlgorithm structure as defined in [RFC 6960 Section 4.4.7.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " PreferredSignatureAlgorithm ::= SEQUENCE {"] # [doc = "    sigIdentifier   AlgorithmIdentifier,"] # [doc = "    certIdentifier  AlgorithmIdentifier OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.4.7.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.4.7.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence , ValueOrd)] # [allow (missing_docs)] pub struct PreferredSignatureAlgorithm { pub sig_identifier : AlgorithmIdentifierOwned , pub cert_identifier : Option < AlgorithmIdentifierOwned > , }
};
}
