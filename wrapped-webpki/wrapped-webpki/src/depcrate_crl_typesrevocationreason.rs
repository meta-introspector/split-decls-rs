// Generated macro for RevocationReason (enum)
macro_rules! Depcrate_crl_typesRevocationReason {
() => {
// Module: crate::crl::types
// Provides: {"RevocationReason"}
// Dependencies: {}
# [doc = " Identifies the reason a certificate was revoked."] # [doc = " See [RFC 5280 §5.3.1][1]"] # [doc = ""] # [doc = " [1]: <https://www.rfc-editor.org/rfc/rfc5280#section-5.3.1>"] # [derive (Debug , Clone , Copy , Hash , Eq , PartialEq)] # [allow (missing_docs)] pub enum RevocationReason { # [doc = " Unspecified should not be used, and is instead assumed by the absence of a RevocationReason"] # [doc = " extension."] Unspecified = 0 , KeyCompromise = 1 , CaCompromise = 2 , AffiliationChanged = 3 , Superseded = 4 , CessationOfOperation = 5 , CertificateHold = 6 , # [doc = " RemoveFromCrl only appears in delta CRLs that are unsupported."] RemoveFromCrl = 8 , PrivilegeWithdrawn = 9 , AaCompromise = 10 , }
};
}
