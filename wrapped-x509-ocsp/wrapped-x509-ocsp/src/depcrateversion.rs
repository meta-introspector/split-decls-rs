// Generated macro for Version (enum)
macro_rules! DepcrateVersion {
() => {
// Module: crate
// Provides: {"Version"}
// Dependencies: {}
# [doc = " OCSP `Version` as defined in [RFC 6960 Section 4.1.1]."] # [doc = ""] # [doc = " ```text"] # [doc = " Version ::= INTEGER { v1(0) }"] # [doc = " ```"] # [doc = ""] # [doc = " [RFC 6960 Section 4.1.1]: https://datatracker.ietf.org/doc/html/rfc6960#section-4.1.1"] # [derive (Clone , Debug , Default , Copy , PartialEq , Eq , Enumerated)] # [asn1 (type = "INTEGER")] # [repr (u8)] pub enum Version { # [doc = " Version 1 (default)"] # [default] V1 = 0 , }
};
}
