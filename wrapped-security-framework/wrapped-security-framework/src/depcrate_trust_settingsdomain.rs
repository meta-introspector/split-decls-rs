// Generated macro for Domain (enum)
macro_rules! Depcrate_trust_settingsDomain {
() => {
// Module: crate::trust_settings
// Provides: {"Domain"}
// Dependencies: {}
# [doc = " Which set of trust settings to query"] # [derive (Debug , Copy , Clone , PartialEq , Eq)] # [repr (u32)] pub enum Domain { # [doc = " Per-user trust settings"] User = kSecTrustSettingsDomainUser , # [doc = " Locally administered, system-wide trust settings"] Admin = kSecTrustSettingsDomainAdmin , # [doc = " System trust settings"] System = kSecTrustSettingsDomainSystem , }
};
}
