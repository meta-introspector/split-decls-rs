// Generated macro for RdnSequence (struct)
macro_rules! Depcrate_nameRdnSequence {
() => {
// Module: crate::name
// Provides: {"RdnSequence"}
// Dependencies: {}
# [doc = " X.501 RDNSequence as defined in [RFC 5280 Section 4.1.2.4]."] # [doc = ""] # [doc = " ```text"] # [doc = " RDNSequence ::= SEQUENCE OF RelativeDistinguishedName"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1.2.4]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1.2.4"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Default , PartialEq , Eq , Hash)] pub struct RdnSequence (Vec < RelativeDistinguishedName >) ;
};
}
