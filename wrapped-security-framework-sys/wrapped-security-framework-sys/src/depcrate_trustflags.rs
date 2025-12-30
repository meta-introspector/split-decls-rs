// Generated macro for flags (module)
macro_rules! Depcrate_trustflags {
() => {
// Module: crate::trust
// Provides: {"flags"}
// Dependencies: {}
# [cfg (target_os = "macos")] mod flags { pub type SecTrustOptionFlags = u32 ; pub const kSecTrustOptionAllowExpired : SecTrustOptionFlags = 0x0000_0001 ; pub const kSecTrustOptionLeafIsCA : SecTrustOptionFlags = 0x0000_0002 ; pub const kSecTrustOptionFetchIssuerFromNet : SecTrustOptionFlags = 0x0000_0004 ; pub const kSecTrustOptionAllowExpiredRoot : SecTrustOptionFlags = 0x0000_0008 ; pub const kSecTrustOptionRequireRevPerCert : SecTrustOptionFlags = 0x0000_0010 ; pub const kSecTrustOptionUseTrustSettings : SecTrustOptionFlags = 0x0000_0020 ; pub const kSecTrustOptionImplicitAnchors : SecTrustOptionFlags = 0x0000_0040 ; }
};
}
