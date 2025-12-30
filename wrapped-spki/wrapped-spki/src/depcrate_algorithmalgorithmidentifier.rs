// Generated macro for AlgorithmIdentifier (struct)
macro_rules! Depcrate_algorithmAlgorithmIdentifier {
() => {
// Module: crate::algorithm
// Provides: {"AlgorithmIdentifier"}
// Dependencies: {}
# [doc = " X.509 `AlgorithmIdentifier` as defined in [RFC 5280 Section 4.1.1.2]."] # [doc = ""] # [doc = " ```text"] # [doc = " AlgorithmIdentifier  ::=  SEQUENCE  {"] # [doc = "      algorithm               OBJECT IDENTIFIER,"] # [doc = "      parameters              ANY DEFINED BY algorithm OPTIONAL  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.1.2]: https://tools.ietf.org/html/rfc5280#section-4.1.1.2"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Copy , Clone , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] pub struct AlgorithmIdentifier < Params > { # [doc = " Algorithm OID, i.e. the `algorithm` field in the `AlgorithmIdentifier`"] # [doc = " ASN.1 schema."] pub oid : ObjectIdentifier , # [doc = " Algorithm `parameters`."] pub parameters : Option < Params > , }
};
}
