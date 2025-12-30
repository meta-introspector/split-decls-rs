// Generated macro for Error (enum)
macro_rules! Depcrate_request_builderError {
() => {
// Module: crate::request::builder
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Debug)] # [non_exhaustive] pub enum Error { # [doc = " ASN.1 DER-related errors."] Asn1 (der :: Error) , # [doc = " Public key errors propagated from the [`spki::Error`] type."] PublicKey (spki :: Error) , # [doc = " Signing error propagated for the [`signature::Error`] type."] Signature (signature :: Error) , # [doc = " Each RelativeDistinguishedName MUST contain exactly one AttributeTypeAndValue."] NonUniqueRdn , # [doc = " Each Name MUST NOT contain more than one instance of a given"] # [doc = " AttributeTypeAndValue across all RelativeDistinguishedNames unless explicitly"] # [doc = " allowed in these Requirements"] NonUniqueATV , # [doc = " Non-ordered attribute or invalid attribute"] InvalidAttribute { # [doc = " Offending [`ObjectIdentifier`]"] oid : ObjectIdentifier , } , # [doc = " Not all required elements were specified"] MissingAttributes , }
};
}
