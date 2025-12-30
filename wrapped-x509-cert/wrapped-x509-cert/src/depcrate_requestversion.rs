// Generated macro for Version (enum)
macro_rules! Depcrate_requestVersion {
() => {
// Module: crate::request
// Provides: {"Version"}
// Dependencies: {}
# [doc = " Version identifier for certification request information."] # [doc = ""] # [doc = " (RFC 2986 designates `0` as the only valid version)"] # [derive (Clone , Debug , Copy , PartialEq , Eq , Enumerated , Default)] # [asn1 (type = "INTEGER")] # [repr (u8)] pub enum Version { # [doc = " Denotes PKCS#8 v1"] # [default] V1 = 0 , }
};
}
