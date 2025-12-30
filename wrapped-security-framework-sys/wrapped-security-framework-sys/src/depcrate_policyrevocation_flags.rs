// Generated macro for revocation_flags (module)
macro_rules! Depcrate_policyrevocation_flags {
() => {
// Module: crate::policy
// Provides: {"revocation_flags"}
// Dependencies: {}
mod revocation_flags { use super :: CFOptionFlags ; pub const kSecRevocationOCSPMethod : CFOptionFlags = 1 << 0 ; pub const kSecRevocationCRLMethod : CFOptionFlags = 1 << 1 ; pub const kSecRevocationPreferCRL : CFOptionFlags = 1 << 2 ; pub const kSecRevocationRequirePositiveResponse : CFOptionFlags = 1 << 3 ; pub const kSecRevocationNetworkAccessDisabled : CFOptionFlags = 1 << 4 ; pub const kSecRevocationUseAnyAvailableMethod : CFOptionFlags = kSecRevocationOCSPMethod | kSecRevocationCRLMethod ; }
};
}
