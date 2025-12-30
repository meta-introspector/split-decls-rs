// Generated macro for Signature (struct)
macro_rules! Depcrate_requestSignature {
() => {
// Module: crate::request
// Provides: {"Signature"}
// Dependencies: {}
# [doc = " Signature structure as defined in [RFC 6960 Section 4.1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " Signature ::= SEQUENCE {"] # [doc = "    signatureAlgorithm      AlgorithmIdentifier,"] # [doc = "    signature               BIT STRING,"] # [doc = "    certs                  [0] EXPLICIT SEQUENCE OF Certificate OPTIONAL }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1"] # [derive (Clone , Debug , Eq , PartialEq , Sequence)] # [allow (missing_docs)] pub struct Signature < P : Profile + 'static = Rfc5280 > { pub signature_algorithm : AlgorithmIdentifierOwned , pub signature : BitString , # [asn1 (context_specific = "0" , optional = "true" , tag_mode = "EXPLICIT")] pub certs : Option < Vec < CertificateInner < P > > > , }
};
}
