// Generated macro for other_10268 (other)
macro_rules! Depcrate_shared_sspiother_10268 {
() => {
// Module: crate::shared::sspi
// Provides: {"other_10268"}
// Dependencies: {}
extern "system" { pub fn SspiPromptForCredentialsW (pszTargetName : PCWSTR , pUiInfo : PCREDUI_INFOW , dwAuthError : c_ulong , pszPackage : PCWSTR , pInputAuthIdentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE , ppAuthIdentity : * mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE , pfSave : * mut c_int , dwFlags : c_ulong ,) -> c_ulong ; pub fn SspiPromptForCredentialsA (pszTargetName : PCSTR , pUiInfo : PCREDUI_INFOA , dwAuthError : c_ulong , pszPackage : PCSTR , pInputAuthIdentity : PSEC_WINNT_AUTH_IDENTITY_OPAQUE , ppAuthIdentity : * mut PSEC_WINNT_AUTH_IDENTITY_OPAQUE , pfSave : * mut c_int , dwFlags : c_ulong ,) -> c_ulong ; }
};
}
