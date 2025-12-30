// Generated macro for impl_714 (impl)
macro_rules! Depcrate_trust_settingsimpl_714 {
() => {
// Module: crate::trust_settings
// Provides: {"impl_714"}
// Dependencies: {}
impl TrustSettingsForCertificate { # [doc = " Create from `kSecTrustSettingsResult*` constant"] fn new (value : i64) -> Self { if value < 0 || value > i64 :: from (u32 :: MAX) { return Self :: Invalid ; } match value as u32 { kSecTrustSettingsResultTrustRoot => Self :: TrustRoot , kSecTrustSettingsResultTrustAsRoot => Self :: TrustAsRoot , kSecTrustSettingsResultDeny => Self :: Deny , kSecTrustSettingsResultUnspecified => Self :: Unspecified , _ => Self :: Invalid , } } }
};
}
