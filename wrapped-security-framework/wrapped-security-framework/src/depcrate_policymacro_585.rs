// Generated macro for macro_585 (macro)
macro_rules! Depcrate_policymacro_585 {
() => {
// Module: crate::policy
// Provides: {"macro_585"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The flags used to specify revocation policy options."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct RevocationPolicy : CFOptionFlags { # [doc = " Perform revocation checking using OCSP (Online Certificate Status Protocol)."] const OCSP_METHOD = kSecRevocationOCSPMethod ; # [doc = " Perform revocation checking using the CRL (Certification Revocation List) method."] const CRL_METHOD = kSecRevocationCRLMethod ; # [doc = " Prefer CRL revocation checking over OCSP; by default, OCSP is preferred."] const PREFER_CRL = kSecRevocationPreferCRL ; # [doc = " Require a positive response to pass the policy."] const REQUIRE_POSITIVE_RESPONSE = kSecRevocationRequirePositiveResponse ; # [doc = " Consult only locally cached replies; do not use network access."] const NETWORK_ACCESS_DISABLED = kSecRevocationNetworkAccessDisabled ; # [doc = " Perform either OCSP or CRL checking."] const USE_ANY_METHOD_AVAILABLE = kSecRevocationUseAnyAvailableMethod ; } }
};
}
