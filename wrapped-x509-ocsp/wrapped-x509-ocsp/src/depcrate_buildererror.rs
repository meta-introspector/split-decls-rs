// Generated macro for Error (enum)
macro_rules! Depcrate_builderError {
() => {
// Module: crate::builder
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Debug)] pub enum Error { # [doc = " ASN.1 DER-related errors"] Asn1 (der :: Error) , # [doc = " Public key errors"] PublicKey (spki :: Error) , # [doc = " Signing errors"] Signature (signature :: Error) , }
};
}
