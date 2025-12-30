// Generated macro for ExpirationPolicy (enum)
macro_rules! Depcrate_crlExpirationPolicy {
() => {
// Module: crate::crl
// Provides: {"ExpirationPolicy"}
// Dependencies: {}
# [doc = " Describes how to handle the nextUpdate field of the CRL (i.e. expiration)."] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub enum ExpirationPolicy { # [doc = " Enforce the verification time is before the time in the nextUpdate field."] # [doc = " Treats an expired CRL as an error condition yielding [Error::CrlExpired]."] Enforce , # [doc = " Ignore the CRL nextUpdate field."] Ignore , }
};
}
