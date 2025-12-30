// Generated macro for Version (enum)
macro_rules! Depcrate_certificateVersion {
() => {
// Module: crate::certificate
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Certificate `Version` as defined in [RFC 5280 Section 4.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " Version  ::=  INTEGER  {  v1(0), v2(1), v3(2)  }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 5280 Section 4.1]: https://datatracker.ietf.org/doc/html/rfc5280#section-4.1"] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] # [derive (Clone , Debug , Copy , PartialEq , Eq , Enumerated)] # [asn1 (type = "INTEGER")] # [repr (u8)] # [derive (Default)] pub enum Version { # [doc = " Version 1 (default)"] # [default] V1 = 0 , # [doc = " Version 2"] V2 = 1 , # [doc = " Version 3"] V3 = 2 , }
};
}
