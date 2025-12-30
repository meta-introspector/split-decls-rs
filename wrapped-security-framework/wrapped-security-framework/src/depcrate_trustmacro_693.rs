// Generated macro for macro_693 (macro)
macro_rules! Depcrate_trustmacro_693 {
() => {
// Module: crate::trust
// Provides: {"macro_693"}
// Dependencies: {}
# [cfg (target_os = "macos")] bitflags :: bitflags ! { # [doc = " The option flags used to configure the evaluation of a `SecTrust`."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TrustOptions : SecTrustOptionFlags { # [doc = " Allow expired certificates (except for the root certificate)."] const ALLOW_EXPIRED = kSecTrustOptionAllowExpired ; # [doc = " Allow CA certificates as leaf certificates."] const LEAF_IS_CA = kSecTrustOptionLeafIsCA ; # [doc = " Allow network downloads of CA certificates."] const FETCH_ISSUER_FROM_NET = kSecTrustOptionFetchIssuerFromNet ; # [doc = " Allow expired root certificates."] const ALLOW_EXPIRED_ROOT = kSecTrustOptionAllowExpiredRoot ; # [doc = " Require a positive revocation check for each certificate."] const REQUIRE_REVOCATION_PER_CERT = kSecTrustOptionRequireRevPerCert ; # [doc = " Use TrustSettings instead of anchors."] const USE_TRUST_SETTINGS = kSecTrustOptionUseTrustSettings ; # [doc = " Treat properly self-signed certificates as anchors implicitly."] const IMPLICIT_ANCHORS = kSecTrustOptionImplicitAnchors ; } }
};
}
