// Generated macro for TrustSettingsForCertificate (enum)
macro_rules! Depcrate_trust_settingsTrustSettingsForCertificate {
() => {
// Module: crate::trust_settings
// Provides: {"TrustSettingsForCertificate"}
// Dependencies: {}
# [doc = " Trust settings for a specific certificate in a specific domain"] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum TrustSettingsForCertificate { # [doc = " Not used"] Invalid , # [doc = " This is a root certificate and is trusted, either explicitly or"] # [doc = " implicitly."] TrustRoot , # [doc = " This is a non-root certificate but is explicitly trusted."] TrustAsRoot , # [doc = " Cert is explicitly distrusted."] Deny , # [doc = " Neither trusted nor distrusted."] Unspecified , }
};
}
