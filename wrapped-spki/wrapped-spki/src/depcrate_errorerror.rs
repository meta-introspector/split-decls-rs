// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error type"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Error { # [doc = " Algorithm parameters are missing."] AlgorithmParametersMissing , # [doc = " ASN.1 DER-related errors."] Asn1 (der :: Error) , # [doc = " Malformed cryptographic key contained in a SPKI document."] # [doc = ""] # [doc = " This is intended for relaying errors related to the raw data contained"] # [doc = " in [`SubjectPublicKeyInfo::subject_public_key`][`crate::SubjectPublicKeyInfo::subject_public_key`]."] KeyMalformed , # [doc = " Unknown algorithm OID."] OidUnknown { # [doc = " Unrecognized OID value found in e.g. a SPKI `AlgorithmIdentifier`."] oid : ObjectIdentifier , } , }
};
}
