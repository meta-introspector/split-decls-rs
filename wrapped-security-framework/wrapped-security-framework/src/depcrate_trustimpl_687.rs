// Generated macro for impl_687 (impl)
macro_rules! Depcrate_trustimpl_687 {
() => {
// Module: crate::trust
// Provides: {"impl_687"}
// Dependencies: {}
impl TrustResult { # [doc = " Indicates a denial by the user, do not proceed."] pub const DENY : Self = Self (kSecTrustResultDeny) ; # [doc = " Indicates a trust policy failure that the user cannot override."] pub const FATAL_TRUST_FAILURE : Self = Self (kSecTrustResultFatalTrustFailure) ; # [doc = " An invalid setting or result."] pub const INVALID : Self = Self (kSecTrustResultInvalid) ; # [doc = " An error not related to trust validation."] pub const OTHER_ERROR : Self = Self (kSecTrustResultOtherError) ; # [doc = " You may proceed."] pub const PROCEED : Self = Self (kSecTrustResultProceed) ; # [doc = " Indicates a trust policy failure that the user can override."] pub const RECOVERABLE_TRUST_FAILURE : Self = Self (kSecTrustResultRecoverableTrustFailure) ; # [doc = " The certificate is implicitly trusted."] pub const UNSPECIFIED : Self = Self (kSecTrustResultUnspecified) ; }
};
}
