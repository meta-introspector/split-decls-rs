// Generated macro for UnknownStatusPolicy (enum)
macro_rules! Depcrate_crlUnknownStatusPolicy {
() => {
// Module: crate::crl
// Provides: {"UnknownStatusPolicy"}
// Dependencies: {}
# [doc = " Describes how to handle the case where a certificate's revocation status is unknown."] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum UnknownStatusPolicy { # [doc = " Treat unknown revocation status permissively, acting as if the certificate were"] # [doc = " not revoked."] Allow , # [doc = " Treat unknown revocation status as an error condition, yielding"] # [doc = " [Error::UnknownRevocationStatus]."] Deny , }
};
}
