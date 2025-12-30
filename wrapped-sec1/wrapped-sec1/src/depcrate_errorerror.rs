// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " ASN.1 DER-related errors."] # [cfg (feature = "der")] Asn1 (der :: Error) , # [doc = " Cryptographic errors."] # [doc = ""] # [doc = " These can be used by EC implementations to signal that a key is"] # [doc = " invalid for cryptographic reasons. This means the document parsed"] # [doc = " correctly, but one of the values contained within was invalid, e.g."] # [doc = " a number expected to be a prime was not a prime."] Crypto , # [doc = " Errors relating to the `Elliptic-Curve-Point-to-Octet-String` or"] # [doc = " `Octet-String-to-Elliptic-Curve-Point` encodings."] PointEncoding , # [doc = " Version errors"] Version , }
};
}
