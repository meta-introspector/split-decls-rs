// Generated macro for impl_712 (impl)
macro_rules! Depcrate_trust_settingsimpl_712 {
() => {
// Module: crate::trust_settings
// Provides: {"impl_712"}
// Dependencies: {}
impl From < Domain > for SecTrustSettingsDomain { # [inline] fn from (domain : Domain) -> Self { match domain { Domain :: User => kSecTrustSettingsDomainUser , Domain :: Admin => kSecTrustSettingsDomainAdmin , Domain :: System => kSecTrustSettingsDomainSystem , } } }
};
}
