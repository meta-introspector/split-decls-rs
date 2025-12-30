// Generated macro for other_559 (other)
macro_rules! Depcrate_trust_settingsother_559 {
() => {
// Module: crate::trust_settings
// Provides: {"other_559"}
// Dependencies: {}
extern "C" { pub fn SecTrustSettingsCopyCertificates (domain : SecTrustSettingsDomain , certsOut : * mut CFArrayRef ,) -> OSStatus ; pub fn SecTrustSettingsCopyTrustSettings (certificateRef : SecCertificateRef , domain : SecTrustSettingsDomain , trustSettings : * mut CFArrayRef ,) -> OSStatus ; pub fn SecTrustSettingsSetTrustSettings (certificateRef : SecCertificateRef , domain : SecTrustSettingsDomain , trustSettingsDictOrArray : CFTypeRef ,) -> OSStatus ; }
};
}
